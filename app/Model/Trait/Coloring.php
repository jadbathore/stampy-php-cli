<?php

namespace Stampy\Model\Trait;

use Stampy\Model\Enum\Text;

trait Coloring
{
    public function color(string $text,string $color,mixed ...$modif):void
    {
        $format = Text::formatColoring($color,$modif);
        echo $format.$text."\e[0m";
    }

    public function textColor(string $text,string $color,mixed ...$modif):string
    {
        $format = Text::formatColoring($color,$modif);
        return $format.$text."\e[0m";
    }
}