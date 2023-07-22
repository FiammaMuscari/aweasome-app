use crate::model::{ProjectForCreate, TaskForCreate};
use crate::Result;
use std::sync::Arc;

use super::ModelStore;

/// Only use while developing. Convenient when to seed the store on start of the application.
pub async fn seed_store_for_dev(model_manager: Arc<ModelStore>) -> Result<()> {
	let ps = ["1", "2"].into_iter().map(|k| {
		(
			k,
			ProjectForCreate {
				name: format!("Projecto {k}"),
			},
		)
	});

	for (_k, project) in ps {
		let project_id = model_manager
			.store()
			.exec_create::<ProjectForCreate>("project", project)
			.await?;

		for i in 1..=10 {
			let done = i == 10;
			let task = TaskForCreate {
				project_id: project_id.clone(),
				title: format!("{i}"),
				desc: None,
				done: Some(done),
			};

			model_manager
				.store()
				.exec_create::<TaskForCreate>("task", task)
				.await?;
		}
	}

	Ok(())
}
