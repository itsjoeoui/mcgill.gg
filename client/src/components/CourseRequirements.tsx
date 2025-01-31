import { Requirements } from '../model/Requirements';

type RequirementsProps = {
  requirements: Requirements;
};

type InfoBlockProps = {
  title: string;
  elements: string[];
};

export const InfoBlock = ({ title, elements }: InfoBlockProps) => {
  return (
    <div>
      <h2 className='mt-1 mb-2 font-semibold leading-none text-gray-700'>
        {title}
      </h2>
      {elements.map((element) => (
        <p className='text-gray-500'>{element}</p>
      ))}
    </div>
  );
};

export const CourseRequirements = ({ requirements }: RequirementsProps) => {
  const hasRequirements =
    requirements.prereqs.length > 0 ||
    requirements.coreqs.length > 0 ||
    requirements.restrictions;

  return hasRequirements ? (
    <div className='w-screen rounded-md bg-slate-50 p-6 md:mx-4 md:mt-10 md:ml-auto md:mr-10 md:w-1/3'>
      <div className='flex-col space-y-3'>
        <div className='m-4 space-y-7'>
          {requirements.prereqs.length > 0 ? (
            <InfoBlock title='Prerequisites' elements={requirements.prereqs} />
          ) : null}
          {requirements.coreqs.length > 0 ? (
            <InfoBlock title='Corequisites' elements={requirements.coreqs} />
          ) : null}
          {requirements.restrictions !== null ? (
            <div>
              <h2 className='mt-1 mb-2 font-semibold leading-none text-gray-700'>
                Restrictions
              </h2>
              <p className='text-gray-500'>{requirements.restrictions}</p>
            </div>
          ) : null}
        </div>
      </div>
    </div>
  ) : null;
};
