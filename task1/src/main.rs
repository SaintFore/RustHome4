use eframe::{egui, App, NativeOptions};
use futures::future::join_all;
use rand::Rng;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use std::fs;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    eframe::run_native(
        "异步任务处理",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx); // 设置字体
            Ok(Box::new(MyApp::default()))
        }),
    )
}

#[derive(Default)]
struct MyApp {
    results: Arc<Mutex<Vec<String>>>, // 线程安全的共享数据
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("启动任务").clicked() {
                let results = Arc::clone(&self.results);
                let ctx_clone = ctx.clone();

                // 使用 tokio::spawn 启动异步任务
                tokio::spawn(async move {
                    let tasks = (0..5).map(|_| perform_task()).collect::<Vec<_>>();
                    let task_results = join_all(tasks).await;

                    // 更新共享的任务结果
                    let mut results_guard = results.lock().unwrap();
                    results_guard.extend(task_results);

                    // 通知 GUI 重绘
                    ctx_clone.request_repaint();
                });
            }

            ui.label("任务结果:");
            let results_guard = self.results.lock().unwrap();
            for result in results_guard.iter() {
                ui.label(result);
            }
        });
    }
}

async fn perform_task() -> String {
    let duration = rand::thread_rng().gen_range(1..5);
    sleep(Duration::from_secs(duration)).await; // 异步延迟
    format!("任务完成，耗时 {} 秒", duration)
}

/// 不改字体会乱码
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 系统字体路径
    let font_path = "C:\\Windows\\Fonts\\msyh.ttc"; // 替换为适合的字体路径

    // 加载字体文件
    if let Ok(font_data) = fs::read(font_path) {
        fonts.font_data.insert(
            "system_font".to_owned(),
            egui::FontData::from_owned(font_data),
        );

        // 设置字体优先级
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "system_font".to_owned());
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "system_font".to_owned());

        ctx.set_fonts(fonts);
    } else {
        eprintln!("无法加载字体文件：{}", font_path);
    }
}
