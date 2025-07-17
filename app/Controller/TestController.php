<?php 
namespace App\Controller;
use App\Model\Abstract\AbstractPrompsController;
use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\Class\Object\Option_CLI;


class TestController extends AbstractPrompsController
{
    #[
        Command('test2'),
        Option(
            [
                '-a'=> new Option_CLI(true,"test option with input"),
                '-b'=> new Option_CLI(false,"test option without input"),
                '-c'=> new Option_CLI(true,"test option with input"),
                '-d'=> new Option_CLI(false,"test option without input")
            ])
        ,Description('Test function')
    ]
    public function test(
        null|string|bool $a,
        null|bool $b,
        null|string|bool $c,
        null|bool $d
    ){
        // Dialoguer::editor("bonjour :");
        // $this->color("test","green");
        var_dump(func_get_args());
        
        \Dialoguer::select("options ?",["a","b"]);
    }

}