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
		Command('COMMAND'),
		Option([
			"-op1"=> new Option_CLI(true,"test option with input"),
			"-op2"=> new Option_CLI(true,"test option with input"),
		]),

		Description('Test function'),
		// StdErr("error.log"),
		// StdIn("input-file.json"),
		StdOut("output-file.txt"),
	]
	public function COMMAND(
		null|bool|string $op1,
		null|bool|string $op2,
){
	echo 'hello';
		/* --- code TODO ---- */
}
}