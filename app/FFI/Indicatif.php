<?php


/**
 * ##  Indicatif
 * An implementation of the Rust crate "Indicatif" allowing a nicely format progressbar.
 */
interface Indicatif {

    /**
     * @param int $length number a iteration througth the progressbar 
     * @throw Error if length is negatif
     */
    public function __construct(int $length);
    /**
     * ## __destruct
     * each time the instance is destruct the progressbar will be finish and clear
     */
    public function __destruct();

    /**
     *  ## increment 
     * @param int $length number a iteration througth the progressbar 
     * @throw Error if length is negatif
     */
    public function increment(int $length = 1);

    /**
     * ## finish
     * end the progressbar
     */
    public function finish();

    /**
     * ## finish
     * end the progressbar an clear the terminal
     */
    public function finishAndClear();
}