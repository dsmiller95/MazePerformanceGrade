using System;
using System.Threading.Tasks;
using Godot;
using MazePerformanceGrade.csharp.maze_scripts.solvers;

namespace MazePerformanceGrade.csharp.maze_scripts;

public partial class MazeEndHandler : Node
{
	[Export] private Camera3D? mazeCam;
	// #WEIRD selecting a reference to a custom C# class in the godot editor does not filter for implementations of that type. it does for gdscript classes. why not C# too? 
	[Export] private MazeReplay? mazeReplay;
	[Export] private RichTextLabel? scoreText;
	
	[Export] private MazeConfig? mazeConfig;
	[Export] private WallCreator? wallCreator;
	 
	[Export] private PathHistory? playerTracker;
	[Export] private Solver[] mazeSolvers = Array.Empty<Solver>();
	[Export] private float replayTotalTimeSeconds = 5;
	
	private void OnBodyEnteredEndMarker(Node3D area)
	{
		_ = PlayEndGame();
	}

	private void _on_end_marker_entered(Node3D area)
	{
		GD.Print("stub end marker entered");
	}

	private bool isEndGameRunning = false;
	private async Task PlayEndGame()
	{
		if(isEndGameRunning)
			return;
		isEndGameRunning = true;
		
		ArgumentNullException.ThrowIfNull(mazeCam);
		ArgumentNullException.ThrowIfNull(playerTracker);
		ArgumentNullException.ThrowIfNull(mazeReplay);
		ArgumentNullException.ThrowIfNull(scoreText);
		ArgumentNullException.ThrowIfNull(wallCreator?.Reachable);
		ArgumentNullException.ThrowIfNull(mazeConfig);
		
		mazeCam.MakeCurrent();
		var playerPathHistory = playerTracker.pathHistory;
		await mazeReplay.BeginPathReplay(playerPathHistory, replayTotalTimeSeconds * 1000);

		scoreText.AppendText("player: " + playerPathHistory.Count + "\n");

		foreach (var solver in mazeSolvers)
		{
			var solution = solver.SolveMaze(wallCreator.Reachable, mazeConfig.entry, 0, mazeConfig.exit);
			
			await mazeReplay.BeginPathReplay(solution, replayTotalTimeSeconds * 1000);
			
			scoreText.AppendText(solver.Name + ": " + solution.Length + "\n");
		}
	}
	
	static StringName completed = new("completed");
	private async Task<Variant> TryAwaitCall(GodotObject obj, string method, params Variant[] args)
	{
		var callResult = obj.Call(method, args);
		
		var asGodotObject = callResult.AsGodotObject();
		if (asGodotObject.GetClass() == "GDScriptFunctionState")
		{
			Variant[] result = await obj.ToSignal(asGodotObject, completed);
			callResult = result[0];
		}

		return callResult;
	}
}