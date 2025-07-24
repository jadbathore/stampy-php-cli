<?php

namespace App\Model\Abstract;

use App\Model\Class\SingleTone\Organisator;
use App\Model\Trait\Coloring;
use Indicatif;

abstract class abstractPrompsController
{
    use Coloring;
    public function __construct(){}

    public function newProgressBar(int $length)
    {
        return new Indicatif($length);
    }
} 