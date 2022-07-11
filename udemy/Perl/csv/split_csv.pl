use strict;
use warnings;

$|=1;

sub main {
  my $input = 'iris.data';

  unless(open(INPUT, $input)) {
    die "\nCannot open $input.\n";
  }

  while(my $line = <INPUT>) {
    my @values = split ',', $line;
    print $values[0] . "\n";
  }

  close INPUT;
}

main();
