import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

const CPIViz = ({ data }) => {
  const svgRef = useRef();

  useEffect(() => {
    const svg = d3.select(svgRef.current);
    // Define graph drawing here using data for CPI and memory usage

    // Sample: visualize CPI as nodes and edges
    svg.selectAll("circle")
      .data(data.cpiNodes)
      .enter()
      .append("circle")
      .attr("cx", d => d.x)
      .attr("cy", d => d.y)
      .attr("r", 10)
      .attr("fill", "blue");

  }, [data]);

  return <svg ref={svgRef} width={800} height={600}></svg>;
};

export default CPIViz;
