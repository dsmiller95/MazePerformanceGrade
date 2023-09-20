using Godot;
using System;
using System.Threading.Tasks;
using MazePerformanceGrade.csharp.helper_classes;

public partial class MazeEndHandler : Node
{
	[Export] private Camera3D mazeCam;
	[Export] private Node mazeReplay;
	[Export] private RichTextLabel scoreText;
	
	[Export] private MazeConfig mazeConfig;
	[Export] private WallCreator wallCreator;
	 
	[Export] private Node playerTracker;
	[Export] private Solver[] mazeSolvers;
	[Export] private float replayTotalTimeSeconds = 5;
	
	private void OnBodyEnteredEndMarker(Node3D area)
	{
		_ = PlayEndGame();
	}

	private void _on_end_marker_entered(Node3D area)
	{
		GD.Print("stub end marker entered");
	}
	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		this.TryMarshalls();
	}

	// Called every frame. 'delta' is the elapsed time since the previous frame.
	public override void _Process(double delta)
	{
	}

	private bool isEndGameRunning = false;
	private async Task PlayEndGame()
	{
		if(isEndGameRunning)
			return;
		isEndGameRunning = true;
		
		
		mazeCam.MakeCurrent();
		var playerPathHistory = playerTracker.Get("path_history");
		mazeReplay.Call("typed_parameter_historic_position", playerPathHistory);
		mazeReplay.Call("generic_parameter", playerPathHistory);
		await TryAwaitCall(
			mazeReplay,
			"begin_path_replay",
			playerPathHistory, replayTotalTimeSeconds * 1000);

		scoreText.AppendText("player: " + playerPathHistory.AsGodotArray().Count + "\n");

		foreach (var solver in mazeSolvers)
		{
			var solution = solver.SolveMaze(wallCreator.reachable, mazeConfig.entry, 0, mazeConfig.exit);
			var solutionAsGodotObjs = HistoricPosition.To(solution);
			var solutionAsVariant = Variant.CreateFrom(solutionAsGodotObjs);

			mazeReplay.Call("typed_parameter", solutionAsVariant);
			mazeReplay.Call("generic_parameter", solutionAsVariant);
			
			await TryAwaitCall(
				mazeReplay,
				"begin_path_replay",
				solutionAsVariant, replayTotalTimeSeconds * 1000);
			
			scoreText.AppendText(solver.Name + ": " + solution.Length + "\n");
		}
	}

	private void TryMarshalls()
	{
		Variant ints = new int[] { 1, 3, 20 };
		var resultA = mazeReplay.Call("typed_parameter_ints", ints);
		var resultB = mazeReplay.Call("generic_parameter", ints);
		
		Variant strings = new string[] {"hello", "world"};
		resultA = mazeReplay.Call("typed_parameter_strings", strings);
		resultB = mazeReplay.Call("generic_parameter", strings);

		var script = GD.Load<GDScript>("res://gdscript/maze_scripts/helper_classes/historic_position.gd");
		Variant historicPositions = new Godot.Collections.Array()
		{
			script.New(Vector2I.Down, 10),
			script.New(Vector2I.Right, 100)
		};
		resultA = mazeReplay.Call("typed_parameter_historic_position", historicPositions);
		resultB = mazeReplay.Call("generic_parameter", historicPositions);
		return;
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
