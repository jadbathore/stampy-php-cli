<?php

/**
 * a throwable class made for catch exception of the binairy
 * @extend Exception
 * 
 */
interface StampyException extends Exception
{
    public function getMessage(): string;
    public function getPrevious(): ?Throwable;
    public function getCode(): int;
    public function getFile(): string;
    public function getLine(): int;
    public function getTrace(): array;
    public function getTraceAsString(): string;
    public function getFormatMessage():void;
}