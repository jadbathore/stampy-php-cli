<?php
namespace Stampy\Controller;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Class\Object\Option_CLI;
use \Dialoguer;

class BinController extends AbstractPrompsController
{
    #[
        Command('test'),
        Option(
            [
                '-test'=> new Option_CLI(true,"test option with input"),
                '-b'=> new Option_CLI(false,"test option without input")
            ])
        ,Description('Test function')
    ]
    public function test(
        null|string|bool $test,
        null|bool $b
    ){
 
    }

    #[
        Command('debug'),
        Description("Special method return this when the command"
        ."\nis not in system or no input has been prompts")
    ]
    public function debug(callable $script)
    {
        $this->color("\nCLI_File_Organisator:\n","green","bold","underline");
        $script();
    }
}