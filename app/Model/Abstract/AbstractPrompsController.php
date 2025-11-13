<?php

namespace Stampy\Model\Abstract;

use ArrayObject;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Trait\Coloring;
use Indicatif;
use Stampy\Model\Class\Singletone\StampyConsole;

abstract class AbstractPrompsController
{
    use Coloring;

    public private(set) StampyConsole $TTY;

    public function __construct() {
        $this->TTY = &StampyConsole::instance();
    }

    public function newProgressBar(int $length)
    {
        return new Indicatif($length);
    }

    public function newOption(bool $input,?string $description = null){
        return new Option_CLI($input,$description);
    }

} 
