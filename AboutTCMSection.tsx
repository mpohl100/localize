const AboutTCMSection = ({ color }: SectionProps) => {
    const [heroAnimationFinished, setHeroAnimationFinished] = useState(false);
    const something = "something";
    return (
        <Section
            id="02"
            classNames={SECTION_COLOR_VARIANTS[color]}>
            <div className="flex flex-col-reverse xl:flex-row justify-end gap-20">
                <AboutTCMHero
                    image={aboutTCMSectionData.heroImage}
                    setHeroAnimationFinished={setHeroAnimationFinished}
                />
            </div>
            <span>blablabla</span>
            <span>{`blablabla`}</span>
            <Marquee content={aboutTCMSectionData.marqueeContent} />
        </Section>
    );
};
export { AboutTCMSection };
