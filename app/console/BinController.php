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

class BinController extends AbstractPrompsController
{
	#[
		Command('zsh'),
		// StdOut('test'),
		Description('---describe your command there---'),
	]
	public function zsh(){
		// $this->TTY->getStdOutTTY()->write("hello");
		// $this->colorOut("test","green");
		/* --- code TODO ---- */
	}
}