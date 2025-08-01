<?php 
namespace Stampy\Controller;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Class\Object\Option_CLI;


class TestController extends AbstractPrompsController
{
    #[
        Command('test1'),
        Option(
            [
                '-a'=> new Option_CLI("a","test option with input"),
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
        var_dump($a,$b,$c,$d);
    }

}