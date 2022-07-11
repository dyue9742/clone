namespace DesignPatterns.Chain {
  public class Creature {
    public string Name;
    public int Attack, Defense;

    public Creature(string name, int attack, int defense) {
      Name = name ?? throw new ArgumentNullException(paramName: nameof(Name));
      Attack = attack;
      Defense = defense;
    }

   public override string ToString()
   {
      return "";
   }
  }
}
