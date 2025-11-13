<?php
namespace StampyConsole;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdOut;
use Stampy\Model\Attributes\StdIn;

class Controller extends AbstractPrompsController
{
	#[
		Command('Command'),
		Description('---describe your command there---'),
		StdErr("error.log"),
		StdIn("composer.json"),
		StdOut("output-file.txt"),
	]
	public function Command(){
		$this->TTY->getStdOutTTY();
		/* --- code TODO ---- */
	}
}