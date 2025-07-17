<?php
namespace App\Controller;

use App\Model\Abstract\AbstractPrompsController;
use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\Class\Object\Option_CLI;
use Dialoguer;

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
        $this->color("test","green");
        var_dump(get_declared_classes());
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