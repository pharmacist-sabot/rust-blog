use leptos::*;

// Code Block
#[component]
fn CodeBlock(code: &'static str) -> impl IntoView {
    view! {
        <pre class="bg-gray-800 text-gray-100 p-4 rounded-lg overflow-x-auto my-4 font-mono text-sm shadow-lg border border-gray-700">
            <code>{code}</code>
        </pre>
    }
}

// Info Box
#[component]
fn InfoBox(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="bg-blue-50 border-l-4 border-brand p-4 my-6 rounded-r-lg shadow-sm">
            <h3 class="font-bold text-brand mb-2">{title}</h3>
            <div class="text-gray-700">{children()}</div>
        </div>
    }
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <div class="container mx-auto max-w-3xl px-4 py-12">
            // Header
            <header class="text-center mb-12">
                <div class="inline-block px-3 py-1 mb-4 text-xs font-semibold tracking-wider text-brand uppercase bg-blue-100 rounded-full">
                    "Built with Rust 🦀"
                </div>
                <h1 class="text-4xl md:text-5xl font-bold mb-4 text-font">
                    "บทเรียน Rust เรื่อง Variables and Mutability"
                </h1>
                <p class="text-xl text-gray-500">
                    "บทความนี้ Render ด้วย WebAssembly"
                </p>
            </header>

            // Content Body
            <article class="prose prose-lg max-w-none text-gray-600">
                <p class="mb-4">
                    "ในภาษา Rust ตัวแปรจะเป็นแบบ "
                    <strong>"ไม่สามารถเปลี่ยนแปลง (Immutable)"</strong>
                    " โดยค่าเริ่มต้น นี่คือวิธีการที่ Rust ช่วยให้เราเขียนโค้ดได้ปลอดภัย"
                </p>

                <h2 class="text-2xl font-bold text-font mt-8 mb-4">
                    "เดี๋ยวเราจะลองเขียนโค้ดที่คอมไพล์ไม่ผ่านดูว่าเพราะอะไร?"
                </h2>
                <p>"ถ้าเราพยายามเปลี่ยนค่าตัวแปรแบบดื้อๆ เลย คอมไพเลอร์จะด่าเราทันที"</p>

                <CodeBlock code=r#"fn main() {
    let x = 5;
    println!("ค่าของ x คือ: {x}");
    x = 6;  // ❌ Error! assign twice to immutable variable
    println!("ค่าของ x คือ: {x}");
    }"# />

                <InfoBox title="ทำไมถึงออกแบบมาแบบนี้?">
                    "การทำให้ตัวแปรไม่เปลี่ยนแปลงโดยค่าเริ่มต้นช่วยลดข้อผิดพลาดจากการเปลี่ยนค่าโดยไม่ตั้งใจ
                    ซึ่งเป็นปัญหาที่พบบ่อยมากในการเขียนโปรแกรม (Concurrency Bug)"
                </InfoBox>

                <h2 class="text-2xl font-bold text-font mt-8 mb-4">
                    "แล้วเราจะทำให้ตัวแปรเปลี่ยนแปลงได้อย่างไร"
                </h2>
                <p>
                    "ถ้าอยากเปลี่ยนค่าจริงๆ ต้องใช้คีย์เวิร์ด "
                    <code class="bg-gray-200 px-1 rounded text-red-500">"mut"</code>
                    " นำหน้าครับ ซึ่ง mut ย่อมาจาก mutable นั่นเอง"
                </p>

                <CodeBlock code=r#"fn main() {
    let mut x = 5; // ✅ ใส่ mut แล้ว
    println!("ค่าของ x คือ: {x}");
    x = 6;
    println!("ค่าของ x คือ: {x}");
    }"# />

                <p>
                  "เมื่อคอมไพล์เราจะได้ ค่าของ x คือ 6 (จะเห็นว่าค่าของ X เปลี่ยนไปแล้ว)"
                </p>

                <div class="my-12 p-8 bg-white rounded-2xl border border-card-border shadow-lg text-center">
                    <h3 class="text-xl font-bold mb-4 text-font">
                        "Demo ของจริงบน Browser"
                    </h3>
                    <p class="mb-4">
                        "ปุ่มข้างล่างนี้คือ Rust State (Signal) ที่ทำงานอยู่จริงๆ บนเครื่องคุณ"
                    </p>

                    <div class="text-6xl font-mono font-bold text-brand mb-6">
                        {move || count.get()}
                    </div>

                    <button
                        class="bg-brand hover:bg-accent text-white font-bold py-3 px-8 rounded-full shadow-lg transition transform hover:-translate-y-1 active:translate-y-0"
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        "เพิ่มค่า (Click Me)"
                    </button>
                    <p class="mt-2 text-sm text-gray-400">
                        "ลองกดดูสิ นี่คือ Rust Wasm ทำงานอยู่"
                    </p>
                </div>

            </article>

            <footer class="text-center mt-20 pt-10 border-t border-gray-200 text-gray-400 text-sm">
                "Created with ❤️ by pharmacist-sabot using Leptos & Rust"
            </footer>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
