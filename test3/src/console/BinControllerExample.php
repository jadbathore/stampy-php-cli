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

class BinControllerExample extends AbstractPrompsController
{
	#[
		Command('co'),

		Description('Test function'),
		StdErr("error.log"),
		StdIn("input-file.json"),
		StdOut("output-file.txt"),
	]
	public function co(){
		/* --- code TODO ---- */
	}
}