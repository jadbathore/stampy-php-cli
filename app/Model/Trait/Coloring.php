<?php

namespace Stampy\Model\Trait;

use Stampy\Model\Enum\Text;
use \ConsoleTTY;
use Stampy\Model\Class\Singletone\StampyConsole;



trait Coloring 
{

    private static function stdoutTTY():ConsoleTTY
    {
        $TTY = &StampyConsole::instance();
        return $TTY->getStdOutTTY();
    }

    private static function stderrTTY():ConsoleTTY
    {
        $TTY = &StampyConsole::instance();
        return $TTY->getStdErrTTY();
    }

    public function textColor(string $text,string $color,mixed ...$modif):string
    {
        $format = Text::formatColoring($color,$modif);
        return $format.$text."\e[0m";
    }

    public function color(string $text,string $color,mixed ...$modif):void
    {
        echo $this->textColor($text,$color,...$modif);
    }

    public function colorOut(string $text,string $color,mixed ...$modif):void
    {
        self::stdoutTTY()->write($this->textColor($text,$color,...$modif));
    }

    public function colorErr(string $text,string $color,mixed ...$modif):void
    {
        self::stderrTTY()->write($this->textColor($text,$color,...$modif));
    }

    
}