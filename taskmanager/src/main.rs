use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const TASKS_FILE: &str = "tasks.json";

#[derive(Parser)]
#[command(name = "taskmanager")]
#[command(about = "Un gestor de tareas por línea de comandos")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        description: String,
    },
    List,
    Complete {
        id: u32,
    },
    Pending {
        id: u32,
    },
    Cancel {
        id: u32,
    },
    Remove {
        id: u32,
    },
    Show {
        id: u32,
    },
    Update {
        id: u32,
        description: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum TaskStatus {
    Pendiente,
    Completado,
    Cancelado,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Pendiente => write!(f, "Pendiente"),
            TaskStatus::Completado => write!(f, "Completado"),
            TaskStatus::Cancelado => write!(f, "Cancelado"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    created_at: String,
    updated_at: String,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
        Task {
            id,
            description,
            status: TaskStatus::Pendiente,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }

    fn update_description(&mut self, description: String) {
        self.description = description;
        self.updated_at = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn load_from_file() -> Result<Self, Box<dyn std::error::Error>> {
        if Path::new(TASKS_FILE).exists() {
            let content = fs::read_to_string(TASKS_FILE)?;
            let manager: TaskManager = serde_json::from_str(&content)?;
            Ok(manager)
        } else {
            Ok(TaskManager::new())
        }
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(TASKS_FILE, content)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) -> u32 {
        let id = self.next_id;
        let task = Task::new(id, description);
        self.tasks.insert(id, task);
        self.next_id += 1;
        id
    }

    fn get_task(&self, id: u32) -> Option<&Task> {
        self.tasks.get(&id)
    }

    fn get_task_mut(&mut self, id: u32) -> Option<&mut Task> {
        self.tasks.get_mut(&id)
    }

    fn remove_task(&mut self, id: u32) -> Option<Task> {
        self.tasks.remove(&id)
    }

    fn list_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values().collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }

    fn list_tasks_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.values()
            .filter(|task| task.status == status)
            .collect();
        tasks.sort_by_key(|task| task.id);
        tasks
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut task_manager = TaskManager::load_from_file()?;

    match &cli.command {
        Some(Commands::Add { description }) => {
            let id = task_manager.add_task(description.clone());
            task_manager.save_to_file()?;
            println!("Tarea añadida con ID: {}", id);
            println!("   Descripción: {}", description);
        }
        
        Some(Commands::List) => {
            let tasks = task_manager.list_tasks();
            if tasks.is_empty() {
                println!(" No hay tareas registradas");
            } else {
                println!(" Lista de tareas:");
                println!("{:-<80}", "");
                for task in tasks {
                    println!("ID: {} | {} | {}", 
                        task.id, 
                        task.status,
                        task.description
                    );
                }
                println!("{:-<80}", "");
                
                let pending = task_manager.list_tasks_by_status(TaskStatus::Pendiente).len();
                let completed = task_manager.list_tasks_by_status(TaskStatus::Completado).len();
                let canceled = task_manager.list_tasks_by_status(TaskStatus::Cancelado).len();
                
                println!(" Resumen: {} pendientes | {} completadas | {} canceladas", 
                    pending, completed, canceled);
            }
        }

        Some(Commands::Complete { id }) => {
            match task_manager.get_task_mut(*id) {
                Some(task) => {
                    let description = task.description.clone();
                    task.update_status(TaskStatus::Completado);
                    task_manager.save_to_file()?;
                    println!(" Tarea {} marcada como completada", id);
                    println!("   Descripción: {}", description);
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Pending { id }) => {
            match task_manager.get_task_mut(*id) {
                Some(task) => {
                    let description = task.description.clone();
                    task.update_status(TaskStatus::Pendiente);
                    task_manager.save_to_file()?;
                    println!(" Tarea {} marcada como pendiente", id);
                    println!("   Descripción: {}", description);
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Cancel { id }) => {
            match task_manager.get_task_mut(*id) {
                Some(task) => {
                    let description = task.description.clone();
                    task.update_status(TaskStatus::Cancelado);
                    task_manager.save_to_file()?;
                    println!(" Tarea {} cancelada", id);
                    println!("   Descripción: {}", description);
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Remove { id }) => {
            match task_manager.remove_task(*id) {
                Some(task) => {
                    task_manager.save_to_file()?;
                    println!("  Tarea {} eliminada permanentemente", id);
                    println!("   Descripción: {}", task.description);
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Show { id }) => {
            match task_manager.get_task(*id) {
                Some(task) => {
                    println!(" Detalles de la tarea {}:", id);
                    println!("{:-<50}", "");
                    println!("ID:          {}", task.id);
                    println!("Descripción: {}", task.description);
                    println!("Estado:      {}", task.status);
                    println!("Creada:      {}", task.created_at);
                    println!("Actualizada: {}", task.updated_at);
                    println!("{:-<50}", "");
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        Some(Commands::Update { id, description }) => {
            match task_manager.get_task_mut(*id) {
                Some(task) => {
                    let old_description = task.description.clone();
                    task.update_description(description.clone());
                    task_manager.save_to_file()?;
                    println!(" Tarea {} actualizada", id);
                    println!("   Descripción anterior: {}", old_description);
                    println!("   Nueva descripción: {}", description);
                }
                None => {
                    eprintln!(" Error: No se encontró la tarea con ID {}", id);
                    std::process::exit(1);
                }
            }
        }

        None => {
            println!(" Gestor de Tareas");
            println!("Usa 'taskmanager --help' para ver todos los comandos disponibles");
            println!("\nComandos principales:");
            println!("  add <descripción>     - Añadir nueva tarea");
            println!("  list                  - Listar todas las tareas");
            println!("  complete <id>         - Marcar tarea como completada");
            println!("  cancel <id>           - Cancelar una tarea");
            println!("  remove <id>           - Eliminar una tarea");
            println!("  show <id>             - Mostrar detalles de una tarea");
            println!("  update <id> <desc>    - Actualizar descripción de una tarea");
        }
    }

    Ok(())
}
