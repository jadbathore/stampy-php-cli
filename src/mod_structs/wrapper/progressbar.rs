use indicatif::ProgressBar;

use crate::mod_traits::builder::class::BuilderWrapper;


#[derive(Default)]
pub struct ProgressBarWrapper
{
    item:Option<ProgressBar>
}

impl ProgressBarWrapper {
    fn new(progress_bar:Option<ProgressBar>)->ProgressBarWrapper {
        ProgressBarWrapper {
            item: progress_bar
        }
    }
}


impl BuilderWrapper for ProgressBarWrapper {
    type OutputType = ProgressBarWrapper;
    type InputType = ProgressBar;
    
    fn set_item(&mut self) {
        self.item = Some(ProgressBar::new(0));
    }

    fn build(self) -> ProgressBarWrapper {
        ProgressBarWrapper::new(self.item)
    }
}