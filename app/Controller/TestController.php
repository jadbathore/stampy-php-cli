<?php 
namespace Stampy\Controller;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdOut;
use Stampy\Model\Attributes\StdIn;


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
        ,Description('Test function'),
        // StdErr("error.log"),
        // StdIn("input-file.json"),
        //StdOut("output-file.txt"),
    ]
    public function test(
        null|string|bool $a,
        null|bool $b,
        null|string|bool $c,
        null|bool $d
    ){
        $this->color("hello for test1","green");
        // $stream = json_decode(stream_get_contents(STDIN));
        // var_dump($stream);
        // echo "a";
        
    }

}