use workspace_domain::{
	entities::{
		BoundedContext,
		Project,
		ProjectScaffolder,
	},
	errors::WorkspaceError,
};

pub struct AddBoundedContextInput {
	pub name: String,
}

impl AddBoundedContextInput {
	pub fn new(name: String) -> Self {
		Self { name }
	}
}

pub struct AddBoundedContext<'a> {
	scaffolder: &'a mut dyn ProjectScaffolder,
}

impl<'a> AddBoundedContext<'a> {
	pub fn new(scaffolder: &'a mut dyn ProjectScaffolder) -> Self {
		Self { scaffolder }
	}

	pub fn execute(&mut self, project: &mut Project, input: AddBoundedContextInput) -> Result<(), WorkspaceError> {
		let bounded_context = BoundedContext::new(input.name)?;
		self.scaffolder.create_bounded_context(project, bounded_context)
	}
}
