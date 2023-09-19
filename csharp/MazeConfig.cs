using Godot;

public partial class MazeConfig : Node
{
	[Export]
	public Vector2I size { get; private set; }
	[Export]
	private Vector2I entry;
	[Export]
	private Vector2I exit;

	public static Vector3 to_3d_from_2d(Vector2I flat)
	{
		return new Vector3(flat.X, 0, flat.Y);
	}

	public bool in_bounds(Vector2I position)
	{
		if (position.X < 0 || position.Y < 0)
			return false;
		if (position.X >= size.X || position.Y >= size.Y)
			return false;
		return true;
	}
	
}
