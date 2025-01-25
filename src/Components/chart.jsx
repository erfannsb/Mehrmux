import React, {useRef, useEffect, useState} from "react";

function Chart() {
    const canvasRef = useRef(null);
    const [data, setData] = useState([["1", 0, 5], ["2", 2, 5], ["3", 5, 6], ["1", 6, 10]]);
    useEffect(() => {
        const colors = [
            "#F38181", // Coral Red
            "#FCE38A", // Lemon Yellow
            "#EAFFD0", // Light Lime Green
            "#95E1D3", // Teal
            "#A8D8EA", // Sky Blue
            "#AA96DA", // Lavender Purple
            "#FC5185", // Watermelon Pink
            "#3FC1C9", // Aqua Blue
            "#FFDD59", // Bright Yellow
            "#FF5722", // Vibrant Orange
            "#C1C8E4", // Soft Periwinkle
            "#FFD460", // Soft Mustard
            "#B8DE6F", // Soft Green
            "#FF6B6B", // Soft Red
            "#6A0572", // Deep Purple
            "#FFE5B4", // Pale Peach
            "#9DDCDC", // Light Cyan
            "#FFB6C1", // Light Pink
            "#F8B195", // Peach Pink
            "#355C7D"  // Slate Blue
        ];
        let max = 0;
        let rows = new Set();
        for (let e of data) {
            rows.add(e[0])
            for (let j of e) {
                if (typeof j == "number") {
                    max = Math.max(max, j);
                }
            }
        }
        console.log(max);
        const canvas = canvasRef.current;
        const context = canvas.getContext("2d");

        // Set the canvas resolution for better quality
        const scale = window.devicePixelRatio || 1; // For high-DPI screens
        const width = canvas.offsetWidth; // Use the displayed width
        const height = canvas.offsetHeight; // Use the displayed height

        // Set the canvas internal dimensions to match the scaled resolution
        canvas.width = width * scale;
        canvas.height = height * scale;

        // Scale the context to account for the increased resolution
        context.scale(scale, scale);

        // Set canvas background color
        canvas.style.backgroundColor = "#0E1420";

        context.fillStyle = "#06101B";

        // Draw an outlined rectangle
        context.fillRect(0, 0, canvas.offsetWidth, 20); // x, y, width, height

        for (let i = 1; i <= max; i++) {
            context.beginPath(); // Start a new path
            context.moveTo((canvas.offsetWidth / max) * i, 20); // Start point (x1, y1)
            context.lineTo((canvas.offsetWidth / max) * i, canvas.offsetHeight); // End point (x2, y2) - match the display width
            context.strokeStyle = "#1F222D"; // Line color
            context.lineWidth = 1; // Line thickness
            context.stroke(); // Draw the line
        }
        context.font = "12px Jetbrains Mono"; // Set font size
        context.fillStyle  = "#7E7BA4";
        for (let j = 0; j <= max; j++) {
            const text = j.toString() + "ms";
            const textWidth = context.measureText(text).width;

            // Calculate the position of each text
            let xPosition = (canvas.offsetWidth / max) * j - textWidth / 2;

            if (j === 0) {
                xPosition += textWidth; // Move the last text 100% of its width to the left
            }
            // If it's the last text, shift it to the left by its width
            if (j === max) {
                xPosition -= textWidth; // Move the last text 100% of its width to the left
            }

            // Draw the text
            context.fillText(text, xPosition, 10);
        }
        const height_of_rows =  30

        for (let i = 0; i < rows.size; i++) {
            context.beginPath();
            context.moveTo(0, height_of_rows*(i+1)+ 20); // Start point (x1, y1)
            context.lineTo(canvas.offsetWidth,  height_of_rows*(i+1) + 20); // End point (x2, y2) - match the display width
            context.strokeStyle = "#1F222D"; // Line color
            context.lineWidth = 1; // Line thickness
            context.stroke(); // Draw the line
        }

        for (let e of data) {
            let y1 = Array.from(rows).indexOf(e[0]) * height_of_rows +  20
            context.fillStyle = colors[Array.from(rows).indexOf(e[0])]
            console.log(y1)
            context.fillRect((canvas.offsetWidth/max) * e[1], y1+5, (canvas.offsetWidth/max) * e[2], 20); // x, y, width, height
            const text = "P" + Array.from(rows).indexOf(e[0]).toString();
            context.fillStyle = "black";
            context.fillText(text, (canvas.offsetWidth/max) * e[1] + 10, y1+20);
        }
    }, []);

    return (
        <canvas
            ref={canvasRef}
            style={{
                height: "100%", // Keep the height responsive
                maxWidth: "90%", // Ensure it stretches to fit the parent container
                display: "block",
                margin: "0 auto",
            }}
        />
    );
}

export default Chart;
