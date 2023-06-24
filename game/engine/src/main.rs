use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(setup)
        .add_system(print_names)
        .add_system(persons_with_jobs)
        .add_system(persons_with_jobs)
        .add_system(persons_job);
    }
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Alex".to_string(),
        },
        Employed { job: Job::Driver },
    ));
    commands.spawn(Person {
        name: "Bob".to_string(),
    });
    commands.spawn((
        Person {
            name: "Charlie".to_string(),
        },
        Employed { job: Job::Medic },
    ));
    commands.spawn((
        Person {
            name: "Daniel".to_string(),
        },
        Employed { job: Job::Sniper },
    ));
    commands.spawn(Person {
        name: "Elan".to_string(),
    });
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name)
    }
}

pub fn persons_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("Person {} has a job.", person.name)
    }
}

pub fn persons_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("Person {} is ready for job.", person.name)
    }
}

pub fn persons_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Medic => "Medic",
            Job::Sniper => "Sniper",
            Job::Driver => "Driver"        };
        
        println!("Person {} is {}", person.name, job_name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Medic,
    Sniper,
    Driver,
}
