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

class zshController extends AbstractPrompsController
{
	#[
		Command('boo'),

		Description('---describe your command there---'),
		// StdErr("error.log"),
		// StdIn("composer.json"),
		// StdOut("output-file.txt"),
	]
	public function Command(){
		var_dump($this->TTY->getStdErrTTY());
		$this->TTY->getStdErrTTY()->write("test");
		$this->TTY->getStdErrTTY()->flush();
		// fwrite(STDERR,"error");
		/* --- code TODO ---- */
	}
}