<?php

interface Indicatif {
    public function __construct(int $length);
    /**
     * each time the instance is destruct the progressbar will be finish and clear
     */
    public function __destruct();
    public function increment(int $length = 1);
    public function finish();
    public function finishAndClear();
}