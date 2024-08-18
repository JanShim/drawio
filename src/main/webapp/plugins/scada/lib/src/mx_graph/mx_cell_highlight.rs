use wasm_bindgen::prelude::*;

use crate::mx_graph::mx_cell_state::MxCellState;

#[wasm_bindgen]
extern "C" {
    pub fn name() -> String;

    pub type MxCellHighlight;

    // #[wasm_bindgen(constructor)]
    // fn new(
    //     graph: mxGraph, 
    //     highlightColor?: string, 
    //     strokeWidth?: number, 
    //     dashed?: boolean,
    // ) -> MxCellHighlight;

    // #[wasm_bindgen(method, getter)]
    // fn number(this: &MyClass) -> u32;


    // highlight(state: mxCellState): void;    
    /**
     * Marks the <markedState> and fires a <mark> event.
     */
    #[wasm_bindgen(method)]
    pub fn highlight(this: &MxCellHighlight, value: Option<MxCellState>);
}


/*

export class mxCellHighlight {
    /**
     * Constructs a cell highlight.
     *
     * @param graph
     * @param highlightColor  default {@link mxConstants.DEFAULT_VALID_COLOR}
     * @param strokeWidth     default {@link mxConstants.HIGHLIGHT_STROKEWIDTH}
     * @param dashed          default false
     */
    constructor(graph: mxGraph, highlightColor?: string, strokeWidth?: number, dashed?: boolean);

    /**
     * Specifies if the highlights should appear on top of everything else in the overlay pane.
     * @default false
     */
    keepOnTop: boolean;

    /**
     * Reference to the enclosing {@link mxGraph}.
     * @default true
     */
    graph: boolean;

    /**
     * Reference to the {@link mxCellState}.
     * @default null
     */
    state: mxCellState;

    /**
     * Specifies the spacing between the highlight for vertices and the vertex.
     * @default 2
     */
    spacing: number;

    /**
     * Holds the handler that automatically invokes reset if the highlight should be hidden.
     * @default null
     */
    // TODO find the right type for resetHandler
    resetHandler: any;

    /**
     * Sets the color of the rectangle used to highlight drop targets.
     *
     * @param {string} color - String that represents the new highlight color.
     */
    setHighlightColor(color: string): void;

    /**
     * Creates and returns the highlight shape for the given state.
     */
    drawHighlight(): void;

    /**
     * Creates and returns the highlight shape for the given state.
     */
    createShape(): mxShape;

    /**
     * Updates the highlight after a change of the model or view.
     */
    getStrokeWidth(state: mxCellState): number;

    /**
     * Updates the highlight after a change of the model or view.
     */
    repaint(): void;

    /**
     * Resets the state of the cell marker.
     */
    hide(): void;

    /**
     * Marks the <markedState> and fires a <mark> event.
     */
    highlight(state: mxCellState): void;

    /**
     * Returns true if this highlight is at the given position.
     */
    isHighlightAt(x: number, y: number): boolean;

    /**
     * Destroys the handler and all its resources and DOM nodes.
     */
    destroy(): void;
  }
}
*/

// #[derive(Deserialize)]
// struct MxCellHighlight  {


// }


// #[wasm_bindgen]
// extern "C" {
//     fn name() -> String;

//     type MxCellHighlight;

//     // #[wasm_bindgen(constructor)]
//     // fn new(
//     //     graph: mxGraph, 
//     //     highlightColor?: string, 
//     //     strokeWidth?: number, 
//     //     dashed?: boolean,
//     // ) -> MxCellHighlight;

//     // #[wasm_bindgen(method, getter)]
//     // fn number(this: &MyClass) -> u32;
//     // #[wasm_bindgen(method, setter)]
//     // fn set_number(this: &MyClass, number: u32) -> MyClass;
//     // #[wasm_bindgen(method)]
//     // fn render(this: &MyClass) -> String;
// }