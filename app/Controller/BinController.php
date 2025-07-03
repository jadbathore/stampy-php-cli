<?php
namespace App\Controller;

use Error;
use App\Model\Abstract\AbstractPrompsController;
use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\Class\Object\Option_CLI;
use Dialoguer;

// use App\Model\Interface\Dialoguer;


class BinController extends AbstractPrompsController
{

    #[
        Command('test'),
        Option(
            [
                '-test'=> new Option_CLI(false,"blabla"),
                '-b'=> new Option_CLI(false,"blabla")
            ])
        ,Description('Test function')
    ]
    public function test(
        null|string|bool $dtest,
        null|string|bool $b
        ){
            \Dialoguer::editor("bonjour :");
            $this->color("test","green");
            // dialoguer::confirm("blabla");
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