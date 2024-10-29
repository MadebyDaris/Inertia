# Adding GUI to the Engine
Using the `egui` immediate mode GUI Rust package. We will need a UI to create and assess the speed of our objects and the norm of the forces.

## How Does `egui` Work

`egui` is an immediate mode GUI library for Rust that allows developers to create graphical user interfaces easily. The immediate mode approach means that the GUI is built and rendered in each frame based on the current application state. This makes it easy to create dynamic interfaces that can change in response to user input or other events.

### Key Features of `egui`

- **Immediate Mode**: UI elements are drawn as they are needed without maintaining a separate state.
- **Integration**: Easily integrates with various backends, making it versatile for different rendering engines.
- **Custom Widgets**: You can create custom widgets and use predefined ones to display information and interact with users.

## Recent Updates

### Implementing the GUI

1. **UI Initialization**: Integrated `egui` into the simulator, setting up a new `MyUI` struct to handle the GUI context and event loop interactions.
  
2. **Rendering Loop Adjustments**: Modified the main rendering loop to include calls to `egui`, ensuring that the UI is updated and drawn each frame.

3. **Widgets**: Created widgets such as `AstralBodyInfoWidget` to display information about celestial objects. The widget shows parameters like speed and forces acting on the bodies.

4. **Event Handling**: Refactored the event handling code to process input events from the `egui` context, allowing interaction with the UI. This includes handling keyboard input to manage camera movements and user interactions with the GUI.

5. **Clear Color Implementation**: Adjusted frame clearing to ensure a consistent background color, resolving rendering issues such as flashing during frame updates.

### Future Improvements

- **Additional Widgets**: Plan to implement more widgets for detailed simulation control, such as sliders for mass and speed adjustments.
- **Data Visualization**: Consider adding visualizations for the forces acting on the bodies to enhance understanding of orbital mechanics.
- **Improved Input Handling**: Enhance input handling for smoother UI interactions and camera control.