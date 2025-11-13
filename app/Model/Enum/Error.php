<?php

namespace Stampy\Model\Enum;

enum Error
{
    case DoubleCommand;
    case ClassNotFound;
    case CommunError;
}