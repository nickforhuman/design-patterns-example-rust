use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct MyExperience {
    components: Vec<ExperienceComponent>,
}
#[derive(Debug, Serialize, Deserialize)]
enum ExperienceComponent {
    Rust,
    Javascript,
    RussianLang,
    EngilishLang,
}

impl MyExperience {
    fn new() -> MyExperience {
        MyExperience {
            components: Vec::new()
        }
    }
    pub fn add_experience(mut self, component: ExperienceComponent) -> Self {
        self.components.push(component);
        self
    }
    pub fn build(self) -> MyExperience {
        self
    }
}

fn main() {
    let builder = MyExperience::new()
        .add_experience(ExperienceComponent::Javascript)
        .add_experience(ExperienceComponent::Rust)
        .add_experience(ExperienceComponent::RussianLang)
        .add_experience(ExperienceComponent::EngilishLang)
        .build();
    println!("{:?}", builder);
}
