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
        Command('test1'),
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
        $controllersNameSpace = new \NamespaceHandler(dirname(__DIR__,2)."/app",\App\Model::class);
        var_dump($controllersNameSpace->resolve());
        $controllersNameSpace->push("Abstract");
        var_dump($controllersNameSpace->resolve());
        $controllersNameSpace->previous();
        var_dump($controllersNameSpace->resolve());
        $controllersNameSpace->push("Enum");
        var_dump($controllersNameSpace->resolve());

    }

}