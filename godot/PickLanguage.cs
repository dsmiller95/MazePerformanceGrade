using Godot;

public partial class PickLanguage : ItemList
{
	[Export] private Godot.Collections.Array< PackedScene> childScenes = new();
	
	public override void _Ready()
	{
		this.Clear();
		foreach (var child in childScenes)
		{
			this.AddItem("scene: " + child.GetState().GetNodeName(0));
		}
		this.ItemClicked += OnItemClicked;
	}

	private void OnItemClicked(long index, Vector2 atposition, long mousebuttonindex)
	{
		var scene = childScenes[(int)index];

		var instance = scene.Instantiate<Node>();

		GetParent().AddChild(instance);
		
		QueueFree();
	}
}
