#[derive(Debug)]
pub struct ResumeData {
    pub personal_info: PersonalInfo,
    pub professional_summary: String,
    pub work_experience: Vec<WorkExperience>,
    pub skills: Vec<String>,
    pub education: Vec<Education>,
}

#[derive(Debug)]
pub struct PersonalInfo {
    pub name: String,
    pub email: String,
    pub linkedin: String,
    pub phone: String,
}

#[derive(Debug)]
pub struct WorkExperience {
    pub company: String,
    pub position: String,
    pub period: String,
    pub location: Option<String>,
    pub description: Vec<Project>,
}

#[derive(Debug, Clone)]
pub struct Project {
    pub project_name: String,
    pub link: Option<String>,
    pub details: String,
    pub technologies: Option<String>,
    pub contributions: Option<Vec<String>>,
    pub features: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Education {
    pub degree: String,
    pub institution: String,
    pub year: String,
}

pub fn get_resume_data() -> ResumeData {
    ResumeData {
        personal_info: PersonalInfo {
            name: "Najam ul Hassan".to_string(),
            email: "realnajmiter@gmail.com".to_string(),
            linkedin: "https://www.linkedin.com/in/najmiter".to_string(),
            phone: "+92-340-5962392".to_string(),
        },
        professional_summary: "A skilled Full Stack Developer with a strong background in building scalable, high-performance web and mobile applications using modern technologies. Proven expertise in React.js, Next.js, TypeScript, and Node.js, with extensive experience in UI/UX design, AI-driven development, and browser extensions. Adept at leading projects from concept to deployment, including AI-powered recruiting platforms, inventory management systems, and productivity tools. Passionate about performance optimization, interactive animations (Framer Motion, GSAP, Three.js), and integrating AI models to enhance user experiences. Strong problem-solving skills, a deep understanding of software architecture, and a track record of delivering innovative solutions. Looking for a challenging role where I can leverage my technical expertise to contribute to impactful projects.".to_string(),
        work_experience: vec![
            WorkExperience {
                company: "IntelliQuarck".to_string(),
                position: "Full Stack Developer".to_string(),
                period: "Jul 2024 - Present".to_string(),
                location: Some("Remote".to_string()),
                description: vec![
                    Project {
                        project_name: "Strivio (web app)".to_string(),
                        link: Some("https://strivio.ai/".to_string()),
                        details: "A recruiting platform with cutting-edge technology aimed to help companies find and recruit talent using AI and manage employees.".to_string(),
                        technologies: Some("Nextjs with Typescript, OpenAi, ElevenLabs, NextUi (HeroUi), Tailwind, Shadcn".to_string()),
                        contributions: Some(vec![
                            "Led development as the lead developer for the entire web app".to_string(),
                            "Built unique features including resume parsing using AI, skill assessment using AI (verbal interview & coding questions with built-in code editor)".to_string(),
                            "Implemented AI-conducted interviews with automatic assessment upon candidate submission".to_string(),
                            "Integrated recording of camera, audio streams, and screen during assessment to monitor candidates".to_string(),
                            "Ensured cross-device compatibility (desktop, mobile, tablets) for integration with the mobile app".to_string(),
                        ]),
                        features: None,
                    },
                    Project {
                        project_name: "Strivio (mobile app)".to_string(),
                        link: Some("https://play.google.com/store/apps/details?id=com.strivio.app".to_string()),
                        details: "Interview Prepping module".to_string(),
                        technologies: None,
                        contributions: Some(vec![
                            "Implemented UI and APIs for the interview preparation feature".to_string(),
                        ]),
                        features: None,
                    },
                    Project {
                        project_name: "Company Website".to_string(),
                        link: Some("https://intelliquarck.com/".to_string()),
                        details: "Developed the entire redesigned company website with animations using framer motion in Nextjs".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                ],
            },
            WorkExperience {
                company: "Knoctal".to_string(),
                position: "Full Stack Developer (Founder)".to_string(),
                period: "Apr 2021 - Present".to_string(),
                location: None,
                description: vec![
                    Project {
                        project_name: "Quvik".to_string(),
                        link: Some("https://play.google.com/store/apps/details?id=com.najmiter.quvik".to_string()),
                        details: "A study app built for matric & intermediate students".to_string(),
                        technologies: None,
                        contributions: None,
                        features: Some(vec![
                            "Annual past papers".to_string(),
                            "Chapter wise past papers".to_string(),
                            "AI generated quizzes for practice".to_string(),
                            "Track student's progress with modern looking charts and graphs".to_string(),
                            "Engage students by keeping a streak count".to_string(),
                            "Daily reminder alerts for study".to_string(),
                        ]),
                    },
                    Project {
                        project_name: "Clipmac".to_string(),
                        link: Some("https://github.com/najmiter/clipmac".to_string()),
                        details: "A small app for MacOS to keep track of the clipboard".to_string(),
                        technologies: None,
                        contributions: None,
                        features: Some(vec![
                            "Clipboard History: Stores a history of copied text items".to_string(),
                            "Quick Access: Open the clipboard history popup using a global shortcut (Control+Shift+Space)".to_string(),
                            "Copy to Clipboard: Easily copy any item from the history back to your clipboard".to_string(),
                            "Simple Interface: Clean and straightforward user interface".to_string(),
                            "Dockless Application: Runs in the background without a dock icon".to_string(),
                            "Auto-launch at startup: Automatically starts when you log in to your Mac (production build only)".to_string(),
                        ]),
                    },
                    Project {
                        project_name: "DoBurji Welfare Society (web app)".to_string(),
                        link: Some("https://www.doburji.com/".to_string()),
                        details: "A town welfare society web app using Nextjs".to_string(),
                        technologies: None,
                        contributions: None,
                        features: Some(vec![
                            "Built-in social platform (share posts with images, react to them, comment on them, share to other platforms)".to_string(),
                            "Role-based donation system (for admins, editors) with automatic image creation and posting".to_string(),
                            "Admin controls for managing users, posts, comments and more".to_string(),
                            "Fully animated landing page using framer motion".to_string(),
                        ]),
                    },
                    Project {
                        project_name: "Beutron".to_string(),
                        link: Some("https://beutron.vercel.app/".to_string()),
                        details: "An inventory management system using Next.js".to_string(),
                        technologies: None,
                        contributions: None,
                        features: Some(vec![
                            "Role-based dashboard with subscription plans allowing multiple employees to manage inventory".to_string(),
                            "Online store launching feature for premium subscribers to take and track orders".to_string(),
                        ]),
                    },
                    Project {
                        project_name: "FontIt".to_string(),
                        link: Some("https://marketplace.visualstudio.com/items?itemName=najmiter.fontit".to_string()),
                        details: "A VS Code extension that allows users to change the editor font family using the command palette".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                    Project {
                        project_name: "Turboin".to_string(),
                        link: Some("https://chromewebstore.google.com/detail/turboin/gnonhllejnghekflninibgaohfbbmekc?authuser=0&hl=en".to_string()),
                        details: "A Chrome extension that allows users to switch tabs similar to IDEs using a command palette, with special commands like calendar viewing".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                    Project {
                        project_name: "Neutrabize".to_string(),
                        link: Some("https://chromewebstore.google.com/detail/neutrabize/ppknddihdfnhmdfloaeamelpinalhppe".to_string()),
                        details: "A Chrome extension for a customized homepage with integrated weather API based on user's geolocation".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                    Project {
                        project_name: "Chitter".to_string(),
                        link: Some("https://najmiter.github.io/Chitter/".to_string()),
                        details: "An assembly language syntax highlighter written in Rust, compiled into WASM for web".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                    Project {
                        project_name: "UOG LMS Beauty Parlor".to_string(),
                        link: Some("https://github.com/najmiter/darkcircles".to_string()),
                        details: "A Chrome extension for custom dark mode on UOG's LMS".to_string(),
                        technologies: None,
                        contributions: None,
                        features: Some(vec![
                            "Integrated Gemini API for suggesting potentially correct answers on quiz pages".to_string(),
                            "Added button functionality to copy quiz questions due to limited Google API usage".to_string(),
                        ]),
                    },
                    Project {
                        project_name: "NutNut".to_string(),
                        link: Some("https://github.com/najmiter/nutnut".to_string()),
                        details: "A subscription-based streaming service app built with ASP.NET and Razor Pages using MSSQL for database".to_string(),
                        technologies: None,
                        contributions: None,
                        features: None,
                    },
                ],
            },
        ],
        skills: vec![
            "React.js".to_string(),
            "React Native".to_string(),
            "Next.js".to_string(),
            "TypeScript".to_string(),
            "Tailwind".to_string(),
            "Git".to_string(),
            "Framer Motion".to_string(),
            "GSAP".to_string(),
            "Three.js".to_string(),
            "ASP.NET".to_string(),
            "Blazor".to_string(),
            "Razor".to_string(),
            "Go".to_string(),
            "Rust".to_string(),
            "Node.js".to_string(),
            "Express".to_string(),
            "NestJS".to_string(),
        ],
        education: vec![
            Education {
                degree: "BSCS".to_string(),
                institution: "".to_string(),
                year: "".to_string(),
            }
        ],
    }
}
