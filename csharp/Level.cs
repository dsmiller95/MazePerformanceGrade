using Godot;

public partial class Level : Node3D
{
	[Export] private bool fastClose;
	
	// Called when the node enters the scene tree for the first time.
	public override void _Ready()
	{
		if (!OS.IsDebugBuild())
		{
			fastClose = false;
		}

		if (fastClose)
		{
			GD.Print("** Fast Close enabled in the 'level.gd' script **");
			GD.Print("** 'Esc' to close 'Shift + F1' to release mouse **");
		}
		SetProcessInput(fastClose);
	}

	public override void _Input(InputEvent @event)
	{
		if(@event.IsActionPressed("ui_cancel"))
			GetTree().Quit();
		if (@event.IsActionPressed("change_mouse_input"))
		{
			switch (Input.MouseMode)
			{
				case Input.MouseModeEnum.Captured:
					Input.MouseMode = Input.MouseModeEnum.Visible;
					break;
				case Input.MouseModeEnum.Visible:
					Input.MouseMode = Input.MouseModeEnum.Captured;
					break;
			}
		}
	}

	public override void _UnhandledInput(InputEvent @event)
	{
		if (@event is InputEventMouseButton { ButtonIndex: MouseButton.Left, Pressed: true })
		{
			Input.MouseMode = Input.MouseModeEnum.Captured;
		}
	}
}
