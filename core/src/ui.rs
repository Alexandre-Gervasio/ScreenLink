use actix_web::{web, App, HttpResponse, HttpServer};
use std::process::{Child, Command};
use std::sync::Mutex;

struct AppState {
    server_process: Mutex<Option<Child>>,
    client_process: Mutex<Option<Child>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        server_process: Mutex::new(None),
        client_process: Mutex::new(None),
    });

    println!("🎛️  KVM Pro UI iniciado em http://127.0.0.1:8080");
    println!("📱 Acesse http://127.0.0.1:8080 no seu navegador");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/", web::get().to(index))
            .route("/api/server/start", web::post().to(start_server))
            .route("/api/server/stop", web::post().to(stop_server))
            .route("/api/client/start", web::post().to(start_client))
            .route("/api/client/stop", web::post().to(stop_client))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    let html = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>🎛️ KVM Pro v1.0.1 - Control Panel</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }

        .container {
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
            max-width: 500px;
            width: 100%;
            padding: 40px;
        }

        h1 {
            color: #333;
            margin-bottom: 10px;
            text-align: center;
            font-size: 28px;
        }

        .subtitle {
            color: #666;
            text-align: center;
            margin-bottom: 40px;
            font-size: 14px;
        }

        .control-section {
            margin-bottom: 40px;
        }

        .section-title {
            font-size: 16px;
            font-weight: 600;
            color: #333;
            margin-bottom: 15px;
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .button-group {
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 12px;
        }

        button {
            padding: 12px 20px;
            border: none;
            border-radius: 8px;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.3s ease;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }

        .btn-start {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
        }

        .btn-start:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 16px rgba(102, 126, 234, 0.4);
        }

        .btn-stop {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }

        .btn-stop:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 16px rgba(245, 87, 108, 0.4);
        }

        button:active {
            transform: translateY(0);
        }

        .status {
            margin-top: 20px;
            padding: 15px;
            border-radius: 8px;
            background: #f5f5f5;
            text-align: center;
            font-size: 12px;
            color: #666;
        }

        .footer {
            text-align: center;
            margin-top: 30px;
            padding-top: 20px;
            border-top: 1px solid #eee;
            font-size: 12px;
            color: #999;
        }

        .loading {
            display: inline-block;
            width: 12px;
            height: 12px;
            margin-right: 8px;
            animation: spin 0.6s linear infinite;
        }

        @keyframes spin {
            to { transform: rotate(360deg); }
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>🎛️ KVM Pro</h1>
        <p class="subtitle">v1.0.1 - Control Panel</p>

        <!-- Servidor -->
        <div class="control-section">
            <div class="section-title">📡 Servidor (Server)</div>
            <div class="button-group">
                <button class="btn-start" onclick="startServer()">🟢 Ligar</button>
                <button class="btn-stop" onclick="stopServer()">🔴 Desligar</button>
            </div>
        </div>

        <!-- Cliente -->
        <div class="control-section">
            <div class="section-title">💻 Cliente (Client)</div>
            <div class="button-group">
                <button class="btn-start" onclick="startClient()">🟢 Conectar</button>
                <button class="btn-stop" onclick="stopClient()">🔴 Desconectar</button>
            </div>
        </div>

        <div class="status">
            <span id="status_text">✅ Pronto para usar</span>
        </div>

        <div class="footer">
            © 2024 KVM Pro • Compartilhamento de teclado e mouse
        </div>
    </div>

    <script>
        function showStatus(message) {
            const status = document.getElementById('status_text');
            status.innerHTML = '<span class="loading">↻</span>' + message;
        }

        function clearStatus() {
            setTimeout(() => {
                document.getElementById('status_text').innerHTML = '✅ Pronto para usar';
            }, 2000);
        }

        async function startServer() {
            showStatus('Iniciando servidor...');
            try {
                const response = await fetch('/api/server/start', { method: 'POST' });
                if (response.ok) {
                    document.getElementById('status_text').innerHTML = '✅ Servidor iniciado';
                } else {
                    document.getElementById('status_text').innerHTML = '❌ Erro ao iniciar servidor';
                }
            } catch (e) {
                document.getElementById('status_text').innerHTML = '❌ Erro: ' + e.message;
            }
            clearStatus();
        }

        async function stopServer() {
            showStatus('Parando servidor...');
            try {
                const response = await fetch('/api/server/stop', { method: 'POST' });
                if (response.ok) {
                    document.getElementById('status_text').innerHTML = '✅ Servidor parado';
                } else {
                    document.getElementById('status_text').innerHTML = '❌ Erro ao parar servidor';
                }
            } catch (e) {
                document.getElementById('status_text').innerHTML = '❌ Erro: ' + e.message;
            }
            clearStatus();
        }

        async function startClient() {
            showStatus('Conectando cliente...');
            try {
                const response = await fetch('/api/client/start', { method: 'POST' });
                if (response.ok) {
                    document.getElementById('status_text').innerHTML = '✅ Cliente conectado';
                } else {
                    document.getElementById('status_text').innerHTML = '❌ Erro ao conectar cliente';
                }
            } catch (e) {
                document.getElementById('status_text').innerHTML = '❌ Erro: ' + e.message;
            }
            clearStatus();
        }

        async function stopClient() {
            showStatus('Desconectando cliente...');
            try {
                const response = await fetch('/api/client/stop', { method: 'POST' });
                if (response.ok) {
                    document.getElementById('status_text').innerHTML = '✅ Cliente desconectado';
                } else {
                    document.getElementById('status_text').innerHTML = '❌ Erro ao desconectar cliente';
                }
            } catch (e) {
                document.getElementById('status_text').innerHTML = '❌ Erro: ' + e.message;
            }
            clearStatus();
        }
    </script>
</body>
</html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

async fn start_server(data: web::Data<AppState>) -> HttpResponse {
    let mut server = data.server_process.lock().unwrap();
    if server.is_none() {
        match Command::new("kvm-pro-server").spawn() {
            Ok(child) => {
                *server = Some(child);
                HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success", "message": "Servidor iniciado"}))
            }
            Err(e) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("Erro: {}", e)})),
        }
    } else {
        HttpResponse::Ok().json(
            serde_json::json!({"status": "warning", "message": "Servidor já está em execução"}),
        )
    }
}

async fn stop_server(data: web::Data<AppState>) -> HttpResponse {
    let mut server = data.server_process.lock().unwrap();
    if let Some(mut child) = server.take() {
        match child.kill() {
            Ok(_) => HttpResponse::Ok()
                .json(serde_json::json!({"status": "success", "message": "Servidor parado"})),
            Err(e) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("Erro: {}", e)})),
        }
    } else {
        HttpResponse::Ok().json(
            serde_json::json!({"status": "warning", "message": "Servidor não está em execução"}),
        )
    }
}

async fn start_client(data: web::Data<AppState>) -> HttpResponse {
    let mut client = data.client_process.lock().unwrap();
    if client.is_none() {
        match Command::new("kvm-pro-client").spawn() {
            Ok(child) => {
                *client = Some(child);
                HttpResponse::Ok()
                    .json(serde_json::json!({"status": "success", "message": "Cliente iniciado"}))
            }
            Err(e) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("Erro: {}", e)})),
        }
    } else {
        HttpResponse::Ok().json(
            serde_json::json!({"status": "warning", "message": "Cliente já está em execução"}),
        )
    }
}

async fn stop_client(data: web::Data<AppState>) -> HttpResponse {
    let mut client = data.client_process.lock().unwrap();
    if let Some(mut child) = client.take() {
        match child.kill() {
            Ok(_) => HttpResponse::Ok()
                .json(serde_json::json!({"status": "success", "message": "Cliente parado"})),
            Err(e) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error", "message": format!("Erro: {}", e)})),
        }
    } else {
        HttpResponse::Ok().json(
            serde_json::json!({"status": "warning", "message": "Cliente não está em execução"}),
        )
    }
}
