// Run with:
// dotnet run 10-factory-2-z3.cs

using Microsoft.Z3;

string[] lines = File.ReadAllLines("input.txt");
int sum = 0;
foreach (string line in lines)
{
    // Parse input
    string[] parts = line.Split(" ").Skip(1).ToArray();
    int[] requirements = parts.Last().Trim('{', '}').Split(',').Select(int.Parse).ToArray();
    int[][] buttons = parts.SkipLast(1).Select(s =>
        s.Trim('(', ')').Split(',').Select(int.Parse).ToArray()).ToArray();

    // Create the Z3 context
    using Context ctx = new();
    
    // Setup equation parameters
    IntExpr[] parameters = Enumerable.Range(0, buttons.Length).Select(b => ctx.MkIntConst($"p{b}")).ToArray();

    // Constraints
    BoolExpr[] parametersMustBeGreaterThanZero = Enumerable.Range(0, buttons.Length).Select(i => ctx.MkGe(parameters[i], ctx.MkInt(0))).ToArray();
    BoolExpr[] buttonPressesMustAddUpToRequirement = Enumerable.Range(0, requirements.Length).Select(i =>
    {
        IntExpr[] buttonsAffectingRequirement = Enumerable.Range(0, buttons.Length)
            .Where(j => buttons[j].Contains(i))
            .Select(b => parameters[b])
            .ToArray();

        return ctx.MkEq(ctx.MkAdd(buttonsAffectingRequirement), ctx.MkInt(requirements[i]));
    }).ToArray();

    // Optimizer
    Optimize opt = ctx.MkOptimize();

    // Optimizer: add previously defined constraints
    opt.Add([.. parametersMustBeGreaterThanZero, .. buttonPressesMustAddUpToRequirement]);

    // Optimizer: the objective is to minimize the sum of all parameters
    ArithExpr sumOfAllParameters = ctx.MkAdd(parameters);
    opt.MkMinimize(sumOfAllParameters);

    if (opt.Check() == Status.SATISFIABLE)
    {
        IntNum result = (IntNum)opt.Model.Evaluate(sumOfAllParameters);
        sum += result.Int;
    }
    else
    {
        throw new InvalidOperationException("Constraints are not satisfiable");
    }
}

Console.WriteLine($"Result: {sum}");
