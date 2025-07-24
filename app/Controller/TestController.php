<?php 
namespace App\Controller;
use App\Model\Abstract\AbstractPrompsController;
use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\Class\Object\Option_CLI;
use Indicatif;

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
        // Dialoguer::editor("bonjour :");
        $this->color("test\n","green");
        $progressBar = $this->newProgressBar(100);
        for($i=0;$i<100;$i++){
            $progressBar->increment();
            if($i == 3){
                unset($progressBar);
                break;
            }
            sleep(1);
        }
    }

}