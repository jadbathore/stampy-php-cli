# Stampy/php-cli

Is a composer plugin that makes it easier to use the PHP command prompt in your project in a clean and organized way.

## installation

> [!TIP]
> Composer will probably ask you if you want to authorize during the installation. By this message

```bash
Package operations: 1 install, 0 updates, 0 removals
stampy/php-cli contains a Composer plugin which is currently not in your allow-plugins config. See https://getcomposer.org/allow-plugins
Do you trust "stampy/php-cli" to execute code and wish to enable it now? (writes "allow-plugins" to composer.json) [y,n,d,?] 
```
Repondez "y" a cette question.

> [!IMPORTANT]
> Stampy/php-cli is a Composer plugin, so it is important that you accept this plugin in your `composer.json` folder in the following way if you have not already done so (composer will automatically format this parameter for you if you have answered "y", otherwise in `composer.json` write)

``` json
"config": {
        "allow-plugins": {
            "stampy/php-cli": true
        }
    },
```

To start the installation of this project through composer, run the command:
```bash
composer require stampy/php-cli
```
After installation, the plugin will check if a pre-compiled version of the stampy library is available for your architecture.
### If a pre-compiled version **exists** for your architecture

Then you would probably see this message

```md
Generating autoload files
? the stampy extension add already a pre-compile binairy you can  ?  ›
  continue as such
  use docker
```
This means that the pre-compiled library is compatible with your architecture so you can now choose between continuing like this or using docker **an executable bash file is also added to the installation so no worries if you want to use the pre-compiled library now and later contain the command prompt**

Subsequently, if you use more than one namespace in your project, stampy will ask you in which namespace you wish to put the controller which will allow you to use the plugin.

It will also create a `.env` folder at the root of your composer project through which you can use environment variables (only for the part using the plugin in your project). 

Finally, an executable file will be created at the root of your project named stampy, from there you can run your command prompt and thanks to that.

> [!CAUTION]
> Do not remove the $NAMESPACE and $ENTRY variables from your .env folder. They are essential for the plugin to function properly.
> You can modify these variables if you wish to activate the plugin from another location.

### If there **doesn't exist** a precompiled version for your architecture
Then you would probably see this message
```md
the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself
using cargo or use docker.If you using cargo make sure you got cargo install (https://rust-lang.org/tools/install/).
If you using docker make sure you docker daemon running [cargo|docker] ?
```
No problem thanks to cargo a compilation of the stampy library will be carried out and will create a personalized executable for your architecture (it must be taken into consideration that because the library uses various cargo extensions it may not be suitable for all types of architecture if this is unfortunately your case I advise you to use the containerized version of the plugin).

### Docker 

To use docker you just had to choose the option [use docker] or docker (if you did not have a pre-compiler). Otherwise a bash script will be created during the installation and you can at any time do the following command to containerize the plugin

```bash
  composer dockerStampy
```
> [!TIP]
> Before running the drown command, make sure you're at the root of your Composer project, i.e.
> the directive where PHP is located.

By following the installation process you arrive at the same result as for the other installation methods: the terminal asks you in which namespace you want to use stampy then creates in your app file a controller, an .env but also a composer.stampy.json file because stampy using docker has a composer.json totally separate from the composer of your application which allows you to completely separate what the plugin can do and what your application can do.

> [!CAUTION]
> Be careful of data overwriting when using this plugin. Due to its early development stage, it is still predetermined for all use cases such as counters.
> This is not yet fully supported for separate projects, so for two separate projects in the same environment, it is important to take this into account.

## Composer.json post-install

After installing the plugin your `composer.json` folder will have changed a little bit first a new namespace will be created
```json
  "autoload": {
        "psr-4": {
            "App\\": "src/",
            "StampyConsole\\": "src/console/"
        }
    },
```
It is inside this Namespace that you could use the controllers allowing you to add the elements allowing you to build your command prompt. By default it is this namespace which is used in your .env folder you would also notice that $ENTRY corresponds to the value of your namespace **for this example** your `.env` folder should look like this
```.env
NAMESPACE=StampyConsole
ENTRY=src/console/
```
Si l'emplacement du controlleur ne vous convient pas vous pouviez toujour le changer par exemple 
```.env
NAMESPACE=App
ENTRY=src/
```

> [!WARNING]
> Because Stampy verifies every class and method in the namespace used, it's important to use only controllers that allow Stampy to be used 
> in this namespace to avoid unnecessary verification during plugin execution.
## Use

Using the plugin is quite simple in the namespace you had assigned in your `.env` folder create a class using the following attributes.
```php
namespace StampyConsole;

use Stampy\Model\Abstract\AbstractPrompsController;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Option;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdOut;
use Stampy\Model\Attributes\StdIn;

class BinController extends AbstractPrompsController
{
	#[
		Command('bin'),/* This attribute allows you to activate
    the command, for example (./stampy bin) */
		Option([
			"-option1" => new Option_CLI(true,"---describe your option there---"),
      /* Each option is linked to a parameter, so "option1" corresponds to the parameter.
      $option1 option1 is either:
      - null I didn't write the option (./stampy bin)
      - bool(true) I wrote the option but didn't pass any input (./stampy bin -option1)
      - string I wrote the option and passed an input (./stampy bin -option1 abc) Here, the value of $option1 = abc
      Since Option_CLI has true as the first parameter, this means that this option accepts input.
      */
			"-option2" => new Option_CLI(false,"---describe your option there---"),
      /*
      So for the case of option 2 given that I have indicated by false in the Option_CLI class that I do not want input for this option $option2 can only be bool or null and therefore does not accept inputs an error is even sent if you all have an input for this file
      */
		]),
    StdOut("output-file.txt"),
    /* Corresponds to the assignment ">" to the call of a php file for example if you want to redistribute a data stream in a particular file in this case all your echo will be redistributed inside and visible output will be redistributed inside the file in question if a file does not exist it will be created automatically */
		Description('---describe your command there---'),
		StdErr("error.log"),
    /* Corresponds to the assignment "2>" when calling a php file, for example, if instead of outputting an error on a terminal you output it to this StdErr file, if a file does not exist it will be created automatically. */
		StdIn("input-file.json"),
    /* Corresponds to the assignment "<" to the call of a php file for example you wish to use from the STDIN stream to give another file for your script if there is no file a bash error will be raised telling you that you must first have a file at the origin to be able to use it correctly */
	]
	public function bin(
		null|bool|string $option1,
		null|bool $option2,
	){
		/* --- example of use ---- */
    stream_get_contents(STDIN); // return the content of the input.json file;
		echo "hello"; // will write in the file output.txt 
		fwrite(STDERR,"log error"); // will write in the file error.log
    var_dump($option1,$option2) 
    /*
      |  input  (in terminal)         |  param      |  value  | 
      |-------------------------------|-------------|---------|
      |  ./stampy bin                 |  $option1   |  null   |
      |-------------------------------|-------------|---------|
      |  ./stampy bin                 |  $option2   |  null   |
      |-------------------------------|-------------|---------|
      |  ./stampy bin -option1        |  $option1   |  true   |
      |-------------------------------|-------------|---------|
      |  ./stampy bin -option2        |  $option2   |  true   |
      |-------------------------------|-------------|---------|
      |  ./stampy bin -option1 abc    |  $option1   |  "abc"  |
      |-------------------------------|-------------|---------|
      |  ./stampy bin -option2 abc    |  $option1   |  ERROR  |
    */
    getenv("ENTRY") // global env variable "ENTRY" which is found in the .env
	}
}
```   