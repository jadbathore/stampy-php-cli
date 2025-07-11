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
                '-test'=> new Option_CLI(true,"test option with input"),
                '-b'=> new Option_CLI(false,"test option without input")
            ])
        ,Description('Test function')
    ]
    public function test(
        null|string|bool $test,
        null|bool $b
        ){
            // Dialoguer::editor("bonjour :");
            $this->color("test","green");
    var_dump(get_declared_classes());

            // dialoguer::confirm("blabla");
        }

}