// ============================================
// IMPORTS (librerías que necesitamos)
// ============================================

// crossterm: maneja el terminal (capturar teclas, control de pantalla)
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode}, // Para leer eventos del teclado
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

// ratatui: librería para dibujar la interfaz TUI
use ratatui::{
    Terminal,
    backend::CrosstermBackend, // Conecta ratatui con crossterm
    layout::{Constraint, Direction, Layout}, // Para dividir la pantalla en secciones
    style::{Color, Modifier, Style}, // Para colores y estilos (negrita, etc)
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap}, // Componentes visuales
};

// Librerías estándar de Rust
use std::io::{self, stdout}; // Para manejar input/output
use std::path::PathBuf; // Para manejar rutas de archivos
use walkdir::WalkDir; // Para recorrer carpetas recursivamente

// ============================================
// ESTRUCTURA DE DATOS
// ============================================

// Struct Book: representa un libro EPUB
// #[derive(Clone)] permite copiar esta estructura fácilmente
#[derive(Clone)]
struct Book {
    name: String,  // Nombre del archivo (ej: "libro.epub")
    path: PathBuf, // Ruta completa del archivo (ej: "/home/user/libro.epub")
}

// ============================================
// FUNCIÓN: Escanear EPUBs en una carpeta
// ============================================

fn scan_epubs(path: &str) -> Vec<Book> {
    // Recorre recursivamente la carpeta (y todas sus subcarpetas)
    WalkDir::new(path)
        .into_iter() // Convierte en iterador para poder usar filter, map, etc
        // filter_map: procesa cada entrada, ignora errores
        .filter_map(|e| e.ok())
        // filter: solo mantiene archivos .epub
        .filter(|e| {
            e.path() // Obtiene la ruta del archivo
                .extension() // Obtiene la extensión (.epub, .txt, etc)
                .and_then(|s| s.to_str()) // Convierte a string
                .map(|s| s.eq_ignore_ascii_case("epub")) // Compara si es "epub" (ignora mayúsculas)
                .unwrap_or(false) // Si no tiene extensión, devuelve false
        })
        // map: transforma cada entrada en un Book
        .map(|e| Book {
            name: e
                .path() // Ruta completa
                .file_name() // Solo el nombre del archivo
                .and_then(|n| n.to_str()) // Convierte a string
                .unwrap_or("Unknown") // Si falla, usa "Unknown"
                .to_string(), // Convierte a String owned
            path: e.path().to_path_buf(), // Guarda la ruta completa
        })
        // collect: convierte el iterador en Vec<Book>
        .collect()
}

// ============================================
// FUNCIÓN: Obtener metadata de un libro
// ============================================

fn get_book_metadata(book: &Book) -> String {
    // Intenta leer la metadata del archivo (tamaño, permisos, etc)
    let metadata = std::fs::metadata(&book.path);

    // match: maneja el resultado (Ok si funciona, Err si falla)
    match metadata {
        Ok(meta) => {
            // Calcula el tamaño en kilobytes (bytes / 1024)
            let size_kb = meta.len() / 1024;

            // format!: crea un String formateado (como template literals en JS)
            format!(
                "Title: {}\n\nPath: {}\n\nSize: {} KB\n\nPress Enter to view in external reader",
                book.name,           // {} se reemplaza por book.name
                book.path.display(), // Muestra la ruta como string
                size_kb              // Tamaño calculado
            )
        }
        Err(_) => "Error reading file metadata".to_string(),
    }
}

// ============================================
// FUNCIÓN PRINCIPAL
// ============================================

fn main() -> io::Result<()> {
    // ============================================
    // PREPARACIÓN DEL TERMINAL
    // ============================================

    // enable_raw_mode: captura todas las teclas sin mostrarlas automáticamente
    enable_raw_mode()?;

    // EnterAlternateScreen: abre una pantalla nueva (al salir, vuelves al terminal normal)
    stdout().execute(EnterAlternateScreen)?;

    // Crea el objeto Terminal que va a dibujar todo
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // ============================================
    // ESTADO DE LA APLICACIÓN
    // ============================================

    let mut should_quit = false; // Flag para saber cuándo salir
    let mut selected = 0; // Índice del libro seleccionado (empieza en 0)
    let books = scan_epubs("/home/damalga/Documentos/Books"); // Escanea la carpeta

    // Si no hay libros, cierra y avisa
    if books.is_empty() {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        println!("No EPUB files found in /home/damalga/Documentos/Books");
        return Ok(());
    }

    // ============================================
    // LOOP PRINCIPAL (se repite hasta salir)
    // ============================================

    while !should_quit {
        // ============================================
        // DIBUJAR LA INTERFAZ
        // ============================================

        terminal.draw(|f| {
            // f = frame, el "lienzo" donde dibujamos

            // ============================================
            // LAYOUT PRINCIPAL: divide en 3 secciones verticales
            // ============================================
            let main_chunks = Layout::default()
                .direction(Direction::Vertical) // Apila verticalmente
                .constraints([
                    Constraint::Length(3), // Header: 3 líneas fijas
                    Constraint::Min(0),    // Body: todo el espacio restante
                    Constraint::Length(3), // Footer: 3 líneas fijas
                ])
                .split(f.size()); // Divide el tamaño total de la pantalla

            // ============================================
            // HEADER (arriba)
            // ============================================
            let header = Paragraph::new("FunkHunt - P2P Book Sharing")
                .style(Style::default().fg(Color::Cyan)) // Texto cyan
                .block(Block::default().borders(Borders::ALL)); // Con bordes
            f.render_widget(header, main_chunks[0]); // Dibuja en la primera sección

            // ============================================
            // BODY: divide en 2 columnas horizontales
            // ============================================
            let body_chunks = Layout::default()
                .direction(Direction::Horizontal) // Lado a lado
                .constraints([
                    Constraint::Percentage(50), // Columna izquierda: 50%
                    Constraint::Percentage(50), // Columna derecha: 50%
                ])
                .split(main_chunks[1]); // Divide la sección del medio

            // ============================================
            // LISTA DE LIBROS (columna izquierda)
            // ============================================

            // Título con contador de libros
            let title = format!("Book List ({})", books.len());

            // Convierte Vec<Book> en Vec<ListItem> (componente visual)
            let items: Vec<ListItem> = books
                .iter() // Itera sobre los libros
                .enumerate() // Añade índice: (0, libro1), (1, libro2)...
                .map(|(i, b)| {
                    // Define el estilo según si está seleccionado
                    let style = if i == selected {
                        // Si es el seleccionado: amarillo y negrita
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        // Si no: blanco normal
                        Style::default().fg(Color::White)
                    };

                    // Crea un ListItem con el nombre del libro y su estilo
                    ListItem::new(b.name.as_str()).style(style)
                })
                .collect(); // Convierte el iterador en Vec

            // Crea el widget List con todos los items
            let list = List::new(items).block(Block::default().borders(Borders::ALL).title(title));

            // Dibuja la lista en la columna izquierda
            f.render_widget(list, body_chunks[0]);

            // ============================================
            // DETALLES DEL LIBRO (columna derecha)
            // ============================================

            // Obtiene la metadata del libro seleccionado
            let details = if !books.is_empty() {
                get_book_metadata(&books[selected])
            } else {
                "No book selected".to_string()
            };

            // Crea un widget de párrafo con la metadata
            let details_widget = Paragraph::new(details)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL).title("Book Details"))
                .wrap(Wrap { trim: true }); // Ajusta el texto si es muy largo

            // Dibuja los detalles en la columna derecha
            f.render_widget(details_widget, body_chunks[1]);

            // ============================================
            // FOOTER (abajo)
            // ============================================
            let footer = Paragraph::new("q: quit | ↑↓: navigate | Enter: open book")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, main_chunks[2]);
        })?; // Fin del draw

        // ============================================
        // MANEJAR INPUT DEL USUARIO
        // ============================================

        // poll: espera 100ms para ver si hay eventos
        if event::poll(std::time::Duration::from_millis(100))? {
            // Si hay evento, léelo
            if let Event::Key(key) = event::read()? {
                // match: maneja diferentes teclas
                match key.code {
                    // Si presiona 'q': activa el flag para salir
                    KeyCode::Char('q') => should_quit = true,

                    // Flecha arriba: sube en la lista (si no está en el tope)
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1; // Resta 1 al índice
                        }
                    }

                    // Flecha abajo: baja en la lista (si no está al final)
                    KeyCode::Down => {
                        // saturating_sub(1) evita overflow si books.len() = 0
                        if selected < books.len().saturating_sub(1) {
                            selected += 1; // Suma 1 al índice
                        }
                    }

                    // Enter: abre el libro con el visor por defecto del sistema
                    KeyCode::Enter => {
                        // xdg-open (Linux) abre archivos con la app asociada
                        let _ = std::process::Command::new("xdg-open")
                            .arg(&books[selected].path) // Pasa la ruta del libro
                            .spawn(); // Ejecuta el comando en background
                    }

                    // Cualquier otra tecla: no hacer nada
                    _ => {}
                }
            }
        }
    } // Fin del while loop

    // ============================================
    // LIMPIEZA: restaurar el terminal
    // ============================================
    disable_raw_mode()?; // Vuelve al modo normal
    stdout().execute(LeaveAlternateScreen)?; // Cierra la pantalla alternativa
    Ok(()) // Retorna éxito
}
