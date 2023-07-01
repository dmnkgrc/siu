import React from "react";
import clsx from "clsx";
import styles from "./styles.module.css";

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: "Simple",
    description: (
      <>
        SMU uses only YAML to configure a project. No need to learn a whole new
        thing.
      </>
    ),
  },
  {
    title: "Open Source",
    description: (
      <>
        SMU is built with developers in mind. It is Open Source and available
        under the MIT license. The code allows for easily adding new tools.
      </>
    ),
  },
  {
    title: "Get started fast",
    description: (
      <>
        When setting up your new laptop or joining a new team, a lot of time is
        lost figuring out what to install and how. SMU allows you to get up and
        running in no time.
      </>
    ),
  },
];

function Feature({ title, description }: FeatureItem) {
  return (
    <div className={clsx("col col--4 padding-vert--lg")}>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
