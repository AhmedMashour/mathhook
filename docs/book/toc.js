// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded affix "><li class="part-title">Getting Started</li><li class="chapter-item expanded "><a href="getting-started/installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li class="chapter-item expanded "><a href="getting-started/quick-start.html"><strong aria-hidden="true">2.</strong> Quick Start</a></li><li class="chapter-item expanded "><a href="getting-started/learning-paths.html"><strong aria-hidden="true">3.</strong> Learning Paths</a></li><li class="chapter-item expanded "><a href="getting-started/basic-usage.html"><strong aria-hidden="true">4.</strong> Basic Usage</a></li><li class="chapter-item expanded "><a href="getting-started/common-patterns.html"><strong aria-hidden="true">5.</strong> Common Patterns</a></li><li class="chapter-item expanded affix "><li class="part-title">Core Concepts</li><li class="chapter-item expanded "><a href="core/expressions.html"><strong aria-hidden="true">6.</strong> Expressions</a></li><li class="chapter-item expanded "><a href="core/symbols-numbers.html"><strong aria-hidden="true">7.</strong> Symbols and Numbers</a></li><li class="chapter-item expanded "><a href="core/functions.html"><strong aria-hidden="true">8.</strong> Functions</a></li><li class="chapter-item expanded "><a href="core/constants.html"><strong aria-hidden="true">9.</strong> Constants</a></li><li class="chapter-item expanded "><a href="core/pattern-matching.html"><strong aria-hidden="true">10.</strong> Pattern Matching</a></li><li class="chapter-item expanded affix "><li class="part-title">Mathematical Operations</li><li class="chapter-item expanded "><a href="operations/simplification.html"><strong aria-hidden="true">11.</strong> Simplification</a></li><li class="chapter-item expanded "><a href="operations/expansion-factoring.html"><strong aria-hidden="true">12.</strong> Expansion and Factoring</a></li><li class="chapter-item expanded "><a href="operations/substitution.html"><strong aria-hidden="true">13.</strong> Substitution</a></li><li class="chapter-item expanded "><a href="operations/differentiation.html"><strong aria-hidden="true">14.</strong> Differentiation</a></li><li class="chapter-item expanded "><a href="operations/integration.html"><strong aria-hidden="true">15.</strong> Integration</a></li><li class="chapter-item expanded "><a href="operations/limits.html"><strong aria-hidden="true">16.</strong> Limits</a></li><li class="chapter-item expanded "><a href="operations/series.html"><strong aria-hidden="true">17.</strong> Series Expansion</a></li><li class="chapter-item expanded "><a href="operations/solving.html"><strong aria-hidden="true">18.</strong> Equation Solving</a></li><li class="chapter-item expanded affix "><li class="part-title">Advanced Features</li><li class="chapter-item expanded "><a href="advanced/complex-numbers.html"><strong aria-hidden="true">19.</strong> Complex Numbers</a></li><li class="chapter-item expanded "><a href="advanced/matrices.html"><strong aria-hidden="true">20.</strong> Matrix Operations</a></li><li class="chapter-item expanded "><a href="advanced/system-solving.html"><strong aria-hidden="true">21.</strong> System Solving</a></li><li class="chapter-item expanded "><a href="advanced/special-functions.html"><strong aria-hidden="true">22.</strong> Special Functions</a></li><li class="chapter-item expanded "><a href="advanced/assumptions.html"><strong aria-hidden="true">23.</strong> Assumptions System</a></li><li class="chapter-item expanded "><a href="advanced/piecewise.html"><strong aria-hidden="true">24.</strong> Piecewise Functions</a></li><li class="chapter-item expanded affix "><li class="part-title">Parser and Formatting</li><li class="chapter-item expanded "><a href="parser/latex.html"><strong aria-hidden="true">25.</strong> LaTeX Parsing</a></li><li class="chapter-item expanded "><a href="parser/wolfram.html"><strong aria-hidden="true">26.</strong> Wolfram Language</a></li><li class="chapter-item expanded "><a href="parser/formatting.html"><strong aria-hidden="true">27.</strong> Expression Formatting</a></li><li class="chapter-item expanded "><a href="parser/custom.html"><strong aria-hidden="true">28.</strong> Custom Parsers</a></li><li class="chapter-item expanded affix "><li class="part-title">Educational Features</li><li class="chapter-item expanded "><a href="educational/step-by-step.html"><strong aria-hidden="true">29.</strong> Step-by-Step Explanations</a></li><li class="chapter-item expanded "><a href="educational/messages.html"><strong aria-hidden="true">30.</strong> Message Registry</a></li><li class="chapter-item expanded "><a href="educational/api.html"><strong aria-hidden="true">31.</strong> Educational API</a></li><li class="chapter-item expanded affix "><li class="part-title">Performance</li><li class="chapter-item expanded "><a href="performance/architecture.html"><strong aria-hidden="true">32.</strong> Architecture Overview</a></li><li class="chapter-item expanded "><a href="performance/simd.html"><strong aria-hidden="true">33.</strong> SIMD Operations</a></li><li class="chapter-item expanded "><a href="performance/parallel.html"><strong aria-hidden="true">34.</strong> Parallel Processing</a></li><li class="chapter-item expanded "><a href="performance/caching.html"><strong aria-hidden="true">35.</strong> Caching Strategies</a></li><li class="chapter-item expanded "><a href="performance/benchmarking.html"><strong aria-hidden="true">36.</strong> Benchmarking</a></li><li class="chapter-item expanded affix "><li class="part-title">Architecture</li><li class="chapter-item expanded "><a href="architecture/principles.html"><strong aria-hidden="true">37.</strong> Design Principles</a></li><li class="chapter-item expanded "><a href="architecture/type-system.html"><strong aria-hidden="true">38.</strong> Type System</a></li><li class="chapter-item expanded "><a href="architecture/function-intelligence.html"><strong aria-hidden="true">39.</strong> Function Intelligence System</a></li><li class="chapter-item expanded "><a href="architecture/memory-layout.html"><strong aria-hidden="true">40.</strong> Memory Layout</a></li><li class="chapter-item expanded "><a href="architecture/thread-safety.html"><strong aria-hidden="true">41.</strong> Thread Safety</a></li><li class="chapter-item expanded affix "><li class="part-title">API Reference</li><li class="chapter-item expanded "><a href="api/core.html"><strong aria-hidden="true">42.</strong> Core API</a></li><li class="chapter-item expanded "><a href="api/algebra.html"><strong aria-hidden="true">43.</strong> Algebra API</a></li><li class="chapter-item expanded "><a href="api/calculus.html"><strong aria-hidden="true">44.</strong> Calculus API</a></li><li class="chapter-item expanded "><a href="api/solver.html"><strong aria-hidden="true">45.</strong> Solver API</a></li><li class="chapter-item expanded "><a href="api/matrix.html"><strong aria-hidden="true">46.</strong> Matrix API</a></li><li class="chapter-item expanded "><a href="api/parser.html"><strong aria-hidden="true">47.</strong> Parser API</a></li><li class="chapter-item expanded affix "><li class="part-title">Language Bindings</li><li class="chapter-item expanded "><a href="bindings/python.html"><strong aria-hidden="true">48.</strong> Python</a></li><li class="chapter-item expanded "><a href="bindings/nodejs.html"><strong aria-hidden="true">49.</strong> Node.js/TypeScript</a></li><li class="chapter-item expanded "><a href="bindings/wasm.html"><strong aria-hidden="true">50.</strong> WebAssembly</a></li><li class="chapter-item expanded affix "><li class="part-title">Contributing</li><li class="chapter-item expanded "><a href="contributing/development.html"><strong aria-hidden="true">51.</strong> Development Guide</a></li><li class="chapter-item expanded "><a href="contributing/testing.html"><strong aria-hidden="true">52.</strong> Testing Strategy</a></li><li class="chapter-item expanded "><a href="contributing/style.html"><strong aria-hidden="true">53.</strong> Code Style</a></li><li class="chapter-item expanded "><a href="contributing/documentation.html"><strong aria-hidden="true">54.</strong> Documentation Standards</a></li><li class="chapter-item expanded "><a href="contributing/correctness.html"><strong aria-hidden="true">55.</strong> Mathematical Correctness</a></li><li class="chapter-item expanded affix "><li class="part-title">Appendix</li><li class="chapter-item expanded "><a href="appendix/notation.html"><strong aria-hidden="true">56.</strong> Mathematical Notation</a></li><li class="chapter-item expanded "><a href="appendix/errors.html"><strong aria-hidden="true">57.</strong> Error Messages</a></li><li class="chapter-item expanded "><a href="appendix/faq.html"><strong aria-hidden="true">58.</strong> FAQ</a></li><li class="chapter-item expanded "><a href="appendix/glossary.html"><strong aria-hidden="true">59.</strong> Glossary</a></li><li class="chapter-item expanded "><a href="appendix/changelog.html"><strong aria-hidden="true">60.</strong> Changelog</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
