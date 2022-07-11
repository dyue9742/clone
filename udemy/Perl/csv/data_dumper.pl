use strict;
use warnings;

use Data::Dumper;

$|=1;

sub main {

  my $input = 'iris.data';

  unless(open(INPUT, $input)) {
    die "Cannot open $input\n";
  }

  <INPUT>;

  while(my $line = <INPUT>) {

    chomp $line;

    # my @values = split ',', $line;
    my @values = split /\s*,\s*/, $line;

    print Dumper(@values);
  }

  close INPUT;
}

main();
