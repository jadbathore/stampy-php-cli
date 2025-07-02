<?php
namespace App\Controller;

use Error;
use App\Model\Abstract\AbstractPrompsController;
use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use Dialoguer;

// use App\Model\Interface\Dialoguer;

class BinController extends AbstractPrompsController
{

    #[
        Command('test'),
        Option(
            [
                '-test'=>'<dtest>',
                '-b'=>true
            ])
        ,Description('Test function')
    ]
    public function test(
        null|string|bool $dtest,
        null|bool $b
        ){
            // \Dialoguer::confirm("aa");
            Dialoguer::input('a');

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