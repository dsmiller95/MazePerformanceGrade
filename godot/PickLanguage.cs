using Godot;

namespace MazePerformanceGrade;

public partial class PickLanguage : ItemList
{
	[Export] private Godot.Collections.Array< PackedScene> childScenes = new();
	
	public override void _Ready()
	{
		Clear();
		foreach (var child in childScenes)
		{
			AddItem("scene: " + child.GetState().GetNodeName(0));
		}
		ItemClicked += OnItemClicked;
	}

	private void OnItemClicked(long index, Vector2 atposition, long mousebuttonindex)
	{
		var scene = childScenes[(int)index];

		var instance = scene.Instantiate<Node>();

		GetParent().AddChild(instance);
		
		QueueFree();
	}
}