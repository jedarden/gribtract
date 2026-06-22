//! `xtask serve` — live benchmark dashboard with SSE streaming.
//!
//! Starts a local HTTP server (default port 7777) that serves:
//!  - `GET /`                → dashboard HTML with a "Run benchmark" button
//!  - `GET /bench-results.json` → current bench-results.json on disk
//!  - `POST /run`            → trigger a full `xtask bench` run; response is an
//!    SSE stream of progress messages followed by a
//!    `done` event that carries the updated JSON.
//!
//! One connection is handled at a time — this is a local dev tool only.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

pub fn run(args: &[String]) {
    let port: u16 = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(7777);

    let addr = format!("127.0.0.1:{port}");
    let listener = TcpListener::bind(&addr)
        .unwrap_or_else(|e| panic!("xtask serve: failed to bind {addr}: {e}"));

    eprintln!("xtask serve: http://localhost:{port}");
    eprintln!("  open http://localhost:{port} in a browser and click ▶ Run benchmark");
    eprintln!("  press Ctrl-C to stop");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("connection error: {e}");
                }
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}

// ── Request dispatch ──────────────────────────────────────────────────────────

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    // Use a clone for BufReader so we keep the original for writing.
    let reader_clone = stream.try_clone()?;
    let mut reader = BufReader::new(reader_clone);

    // Read request line
    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let request_line = request_line.trim_end().to_owned();
    if request_line.is_empty() {
        return Ok(());
    }

    // Drain headers; note Content-Length in case of POST body
    let mut content_length = 0usize;
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let trimmed = line.trim_end();
        if trimmed.is_empty() {
            break;
        }
        if let Some(rest) = trimmed.strip_prefix("Content-Length: ") {
            content_length = rest.trim().parse().unwrap_or(0);
        }
    }
    // Drain any POST body (we don't use it)
    if content_length > 0 {
        use std::io::Read;
        let mut body = vec![0u8; content_length.min(65536)];
        reader.read_exact(&mut body)?;
    }

    let parts: Vec<&str> = request_line.splitn(3, ' ').collect();
    let method = parts.first().copied().unwrap_or("GET");
    let path = parts.get(1).copied().unwrap_or("/");

    match (method, path) {
        ("GET", "/") | ("GET", "/dashboard.html") => serve_live_dashboard(&mut stream),
        ("GET", "/bench-results.json") => serve_bench_json(&mut stream),
        ("POST", "/run") => serve_run_sse(&mut stream),
        _ => serve_404(&mut stream),
    }
}

// ── Handlers ──────────────────────────────────────────────────────────────────

fn serve_live_dashboard(stream: &mut TcpStream) -> std::io::Result<()> {
    let html = build_live_dashboard_html();
    let headers = ok_headers("text/html; charset=utf-8", html.len());
    stream.write_all(headers.as_bytes())?;
    stream.write_all(html.as_bytes())
}

fn serve_bench_json(stream: &mut TcpStream) -> std::io::Result<()> {
    let body = std::fs::read_to_string("bench-results.json").unwrap_or_else(|_| "{}".to_owned());
    let headers = ok_headers("application/json", body.len());
    stream.write_all(headers.as_bytes())?;
    stream.write_all(body.as_bytes())
}

/// POST /run — run the benchmark in a background thread and stream its
/// progress as SSE events.  When done, sends a final `done` event whose
/// `data` field contains a JSON object `{"bench": "<bench-results.json content>"}`.
fn serve_run_sse(stream: &mut TcpStream) -> std::io::Result<()> {
    // SSE headers — no Content-Length (open-ended stream)
    stream.write_all(
        b"HTTP/1.1 200 OK\r\n\
          Content-Type: text/event-stream\r\n\
          Cache-Control: no-cache\r\n\
          Access-Control-Allow-Origin: *\r\n\
          Connection: keep-alive\r\n\r\n",
    )?;
    stream.flush()?;

    let (tx, rx) = mpsc::channel::<String>();

    // Run benchmark in background, forwarding progress to `tx`.
    std::thread::spawn(move || {
        crate::bench::run_with_sender(&[], tx);
    });

    // Forward progress messages as SSE events
    for msg in rx {
        // SSE data lines must not contain raw newlines
        let safe = msg.replace('\n', " ").replace('\r', "");
        let evt = format!("event: progress\ndata: {safe}\n\n");
        if stream.write_all(evt.as_bytes()).is_err() {
            break; // client disconnected
        }
        stream.flush().ok();
    }

    // Benchmark thread has finished (channel closed).  Read the fresh JSON
    // and send a `done` event so the browser can update without a reload.
    let bench_json = std::fs::read_to_string("bench-results.json").unwrap_or_default();
    // Escape the JSON for embedding in an SSE data field (must be one line)
    let bench_json_oneline = bench_json.replace('\n', " ").replace('\r', "");
    let done_payload = format!("{{\"bench\":{bench_json_oneline}}}");
    let evt = format!("event: done\ndata: {done_payload}\n\n");
    stream.write_all(evt.as_bytes()).ok();
    stream.flush().ok();

    Ok(())
}

fn serve_404(stream: &mut TcpStream) -> std::io::Result<()> {
    stream.write_all(
        b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
    )
}

// ── HTML construction ─────────────────────────────────────────────────────────

fn build_live_dashboard_html() -> String {
    let bench_json = std::fs::read_to_string("bench-results.json").unwrap_or_else(|_| "{}".into());
    let history_csv = crate::bench::read_history_for_dashboard();
    let git_sha = crate::bench::get_git_sha();

    // Build the static dashboard then inject the serve-mode panel before </body>
    let mut html = crate::bench::render_dashboard(&bench_json, &history_csv, &git_sha);
    html = html.replace("</body>", &format!("{SERVE_PANEL_HTML}\n</body>"));
    html
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn ok_headers(content_type: &str, content_length: usize) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {content_length}\r\nConnection: close\r\n\r\n"
    )
}

// ── Serve-mode panel HTML + JS ─────────────────────────────────────────────────
//
// This snippet is injected before </body> when serving the live dashboard.
// It adds a floating "Run benchmark" panel with a progress log and SSE client.

const SERVE_PANEL_HTML: &str = r#"
<style>
#serve-panel{position:fixed;bottom:1rem;right:1rem;background:#161b22;border:1px solid #21262d;
  border-radius:8px;padding:1rem;min-width:320px;max-width:420px;z-index:999;box-shadow:0 4px 24px rgba(0,0,0,.5)}
#serve-panel h3{margin:0 0 0.5rem;color:#58a6ff;font-size:0.9rem;font-weight:600}
#run-btn{background:#1a7f37;color:#fff;border:none;padding:6px 16px;border-radius:4px;
  cursor:pointer;font-weight:600;font-size:0.85rem}
#run-btn:disabled{background:#2d333b;color:#8b949e;cursor:not-allowed}
#run-status{font-size:0.78rem;color:#8b949e;margin-left:0.5rem}
#run-log{font-family:'SF Mono',monospace;font-size:0.72rem;max-height:180px;overflow-y:auto;
  white-space:pre-wrap;color:#c9d1d9;background:#0d1117;padding:0.5rem;border-radius:4px;
  margin-top:0.5rem;min-height:48px;border:1px solid #21262d}
</style>
<div id="serve-panel">
  <h3>Live benchmark</h3>
  <div style="display:flex;align-items:center">
    <button id="run-btn" onclick="runBench()">&#9654; Run benchmark</button>
    <span id="run-status"></span>
  </div>
  <div id="run-log">Click "Run benchmark" to start.</div>
</div>
<script>
(function(){
  function runBench(){
    var btn=document.getElementById('run-btn');
    var status=document.getElementById('run-status');
    var log=document.getElementById('run-log');
    btn.disabled=true;
    status.textContent='running…';
    log.textContent='';

    fetch('/run',{method:'POST'})
      .then(function(resp){
        if(!resp.body){throw new Error('No response body (SSE not supported?)');}
        var reader=resp.body.getReader();
        var decoder=new TextDecoder();
        var buf='';
        function pump(){
          return reader.read().then(function(r){
            if(r.done){onDone(null);return;}
            buf+=decoder.decode(r.value,{stream:true});
            var parts=buf.split('\n\n');
            buf=parts.pop();
            parts.forEach(function(evt){
              var lines=evt.split('\n');
              var evtType='message',data='';
              lines.forEach(function(l){
                if(l.startsWith('event: '))evtType=l.slice(7);
                else if(l.startsWith('data: '))data=l.slice(6);
              });
              if(evtType==='progress'){
                log.textContent+=data+'\n';
                log.scrollTop=log.scrollHeight;
              }else if(evtType==='done'){
                try{
                  var d=JSON.parse(data);
                  onDone(d.bench?JSON.parse(d.bench):null);
                }catch(e){onDone(null);}
              }
            });
            return pump();
          });
        }
        return pump();
      })
      .catch(function(e){
        log.textContent+='fetch error: '+e+'\n';
        btn.disabled=false;
        status.textContent='error';
      });
  }

  function onDone(bench){
    var btn=document.getElementById('run-btn');
    var status=document.getElementById('run-status');
    var log=document.getElementById('run-log');
    log.textContent+='--- benchmark complete ---\n';
    status.textContent='done';
    btn.disabled=false;
    if(bench){
      window.BENCH=bench;
      log.textContent+='Reloading page with fresh results…\n';
      setTimeout(function(){window.location.reload();},800);
    }
  }

  window.runBench=runBench;
}());
</script>
"#;
