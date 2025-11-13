<?php

namespace Stampy\Model\Abstract;

use Stampy\Model\Class\SingleTone\StampyConsole;
use \ConsoleTTY;

abstract class AbstractError
{
    protected ConsoleTTY $stderr_tty;

    protected function __construct() {
        $stampyConsoleOInstance = &StampyConsole::instance();
        $this->stderr_tty = $stampyConsoleOInstance->getStdErrTTY();
    }
}