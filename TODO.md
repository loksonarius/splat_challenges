# To Do's

- Update handlers to return custom error messages
- Augment Challenge model
  - Delineate weapon, map, mode, and category tags to attach to Challenges
  - Augment challenge listing with query params for tags
- Move static Challenge functions into a ChallengeRepo module
- Let error catchers display error details from status
  - Replace error catchers with custom Responses
- Migrate from SQLite to MariaDB
  - Set up dev env tools needed to run MariaDB locally
  - Make connection string env var
  - Enable consumption of DB connection pool instead of single connection
- Move challenges into own directory
  - Create models, schema, routes, and handlers in own modules
  - Have Rocket instance modifier that plugs in Challenge routes, handlers, etc
- Make distributables
- Make Dockerfile for deployment
- Set up CI for all deliverables
- Set up token auth
  - Research token authentication and generation
  - Research Rocket auth options and libs
- Enable challenge progress tracking
  - Add counter field for Challenges
  - Add counter unit field for Challenges
  - Enable Challenge counter levels
  - Augment challenge listing with query params for complete/in-progress/new
